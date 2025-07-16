pub mod read_kdl {
    use std::collections::HashMap;

    use crate::game::game::*;
    use kdl::*;

    pub fn reality_from_kdl(data: String) -> Option<Reality> {
        let mut reality = Reality::default();
        let doc = data.parse::<KdlDocument>().ok()?;
        for node in doc.nodes() {
            match node.name().value() {
                "event" => {
                    reality.world.events.insert(
                        Pos {
                            x: node.entries().get(0)?.value().as_integer()? as Int,
                            y: node.entries().get(1)?.value().as_integer()? as Int,
                        },
                        Event {
                            preview: String::from(
                                node.children()?
                                    .get("preview")?
                                    .entries()
                                    .get(0)?
                                    .value()
                                    .as_string()?,
                            ),
                            tree: parse_tree(node.children()?.get("tree")?)?,
                        },
                    );
                }
                _ => {}
            }
        }
        Some(reality)
    }

    fn parse_tree(ev: &KdlNode) -> Option<EventTree> {
        let mut tree = EventTree {
            node: EventNode {
                text: String::new(),
                items: Vec::new(),
            },
            sub: HashMap::new(),
        };
        for node in ev.children()?.nodes() {
            match node.name().value() {
                "tree" => {
                    let act;
                    let arg = node.entries().get(0)?.value();
                    match arg.is_string() {
                        true => act = Action::Simple(String::from(arg.as_string()?)),
                        false => act = Action::UseItem(arg.as_integer()? as u16),
                    }
                    tree.sub.insert(act, parse_tree(node)?);
                }
                "text" => {
                    tree.node.text = String::from(node.entries().get(0)?.value().as_string()?);
                }
                "item" => {
                    tree.node.items.push(parse_item(node.children()?)?);
                }
                _ => {}
            }
        }
        Some(tree)
    }
    fn parse_item(it: &KdlDocument) -> Option<Item> {
        Some(Item {
            id: it.get("id")?.entries().get(0)?.value().as_integer()? as u16,
            name: String::from(it.get("name")?.entries().get(0)?.value().as_string()?),
            item_type: match it.get("type")?.entries().get(0)?.value().as_string()? {
                "N" => ItemType::Normal,
                "W" => ItemType::Weapon(WeaponData {
                    damage: it.get("type")?.entries().get(1)?.value().as_integer()? as u16,
                }),
                _ => return None,
            },
        })
    }
}
