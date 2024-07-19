use std::collections::HashMap;

use crate::emit_signal;
use godot::prelude::*;

use super::gem_fragment::GemFragment;
use super::gem_fragment_link::*;

#[derive(Debug)]
enum GemIssueKind {
    FragmentNotValid,
    LinkNotValid,
}

struct GemIssue {
    kind: GemIssueKind,
    msg: String,
}

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct Gem {
    #[base]
    base: Base<Resource>,

    #[export]
    #[var(set = set_input_fragments, get)]
    pub input_fragments: Array<Option<Gd<GemFragment>>>,

    #[export]
    #[var(set = set_driver_fragments, get)]
    pub driver_fragments: Array<Option<Gd<GemFragment>>>,

    #[export]
    #[var(set = set_driver_fragments, get)]
    pub output_fragments: Array<Option<Gd<GemFragment>>>,

    #[export]
    #[var(set = set_input_driver_links, get)]
    pub input_driver_links: Array<Option<Gd<GemFragmentLink>>>,

    #[export]
    #[var(set = set_driver_output_links, get)]
    pub driver_output_links: Array<Option<Gd<GemFragmentLink>>>,

    #[export]
    pub issues: Array<GString>,
}

#[godot_api]
pub impl IResource for Gem {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base,
            input_fragments: Array::new(),
            driver_fragments: Array::new(),
            output_fragments: Array::new(),
            input_driver_links: Array::new(),
            driver_output_links: Array::new(),
            issues: Array::new(),
        }
    }
}

#[godot_api]
impl Gem {
    #[signal]
    pub fn data_changed(&self);

    #[func]
    pub fn rs_init(&mut self) {
        self.validate();
    }

    pub fn get_fragments(&self) -> HashMap<(usize, usize), Option<Gd<GemFragment>>> {
        let fragment_arrays = vec![
            self.input_fragments.clone(),
            self.driver_fragments.clone(),
            self.output_fragments.clone(),
        ];
        fragment_arrays
            .iter()
            .enumerate()
            .flat_map(|(arr_idx, array)| {
                array
                    .iter_shared()
                    .enumerate()
                    .map(move |(frag_idx, opt_frag)| ((arr_idx.clone(), frag_idx), opt_frag))
            })
            .collect()
    }

    pub fn get_links(&self) -> HashMap<(usize, usize), Option<Gd<GemFragmentLink>>> {
        let link_arrays = vec![
            self.input_driver_links.clone(),
            self.driver_output_links.clone(),
        ];
        link_arrays
            .iter()
            .enumerate()
            .flat_map(|(arr_idx, array)| {
                array
                    .iter_shared()
                    .enumerate()
                    .map(move |(link_idx, opt_link)| ((arr_idx.clone(), link_idx), opt_link))
            })
            .collect()
    }

    pub fn get_subresources(&self) -> Vec<Gd<Resource>> {
        let fragments = self.get_fragments();
        let frag_resources = fragments.iter().filter_map(|(_id, frag_opt)| {
            if let Some(frag_ref) = frag_opt {
                Some(frag_ref.clone().upcast())
            } else {
                None
            }
        });

        let links = self.get_links();
        let link_resources = links.iter().filter_map(|(_id, link_opt)| {
            if let Some(link_ref) = link_opt {
                Some(link_ref.clone().upcast())
            } else {
                None
            }
        });
        frag_resources.chain(link_resources).collect()
    }

    #[func]
    fn set_input_fragments(&mut self, input_fragments: Array<Option<Gd<GemFragment>>>) {
        godot_print!("Gem::set_input_fragments");
        self.input_fragments = input_fragments;
        self.handle_data_changed()
    }

    #[func]
    fn set_driver_fragments(&mut self, driver_fragments: Array<Option<Gd<GemFragment>>>) {
        self.driver_fragments = driver_fragments;
        self.handle_data_changed()
    }

    #[func]
    fn set_output_fragments(&mut self, output_fragments: Array<Option<Gd<GemFragment>>>) {
        self.output_fragments = output_fragments;
        self.handle_data_changed()
    }

    #[func]
    fn set_input_driver_links(&mut self, input_driver_links: Array<Option<Gd<GemFragmentLink>>>) {
        self.input_driver_links = input_driver_links;
        self.handle_data_changed()
    }

    #[func]
    fn set_driver_output_links(&mut self, driver_output_links: Array<Option<Gd<GemFragmentLink>>>) {
        self.driver_output_links = driver_output_links;
        self.handle_data_changed()
    }

    #[func]
    fn handle_data_changed(&mut self) {
        godot_print!("Gem::handle_data_changed");
        self.validate();
        self.observe_subresources();
        self.base_mut().notify_property_list_changed();
        emit_signal!(self, "data_changed");
    }

    pub fn validate(&mut self) {
        godot_print!("Gem::validate");
        self.issues.clear();

        self.validate_input_fragments();
        self.validate_driver_fragments();
        self.validate_output_fragments();
        self.validate_input_driver_links();
        self.validate_driver_output_links();
    }

    pub fn validate_input_fragments(&mut self) {
        godot_print!("Gem::validate_input_fragments");
        let frags = self.input_fragments.clone();
        self.validate_fragments(
            frags,
            |frag_ref| frag_ref.clone().bind().is_input(),
            GemIssue {
                kind: GemIssueKind::FragmentNotValid,
                msg: "Fragment is not an Input Fragment".into(),
            },
        )
    }

    pub fn validate_driver_fragments(&mut self) {
        self.validate_fragments(
            self.driver_fragments.clone(),
            |frag_ref| frag_ref.clone().bind().is_driver(),
            GemIssue {
                kind: GemIssueKind::FragmentNotValid,
                msg: "Fragment is not a Driver fragment".into(),
            },
        )
    }

    pub fn validate_output_fragments(&mut self) {
        self.validate_fragments(
            self.output_fragments.clone(),
            |frag_ref| frag_ref.clone().bind().is_output(),
            GemIssue {
                kind: GemIssueKind::FragmentNotValid,
                msg: "Fragment is not an Output Fragment".into(),
            },
        )
    }

    #[func]
    pub fn validate_input_driver_links(&mut self) -> bool {
        let result = verify_links(
            &self.input_driver_links,
            &self.input_fragments,
            &self.driver_fragments,
        );
        self.add_issues_from_link_validation(
            &result,
            GemIssue {
                kind: GemIssueKind::LinkNotValid,
                msg: "Input Link is not valid".into(),
            },
        );
        result.is_ok()
    }

    #[func]
    pub fn validate_driver_output_links(&mut self) -> bool {
        let result = verify_links(
            &self.driver_output_links,
            &self.driver_fragments,
            &self.output_fragments,
        );
        self.add_issues_from_link_validation(
            &result,
            GemIssue {
                kind: GemIssueKind::LinkNotValid,
                msg: "Input Link is not valid".into(),
            },
        );
        result.is_ok()
    }

    fn validate_fragments(
        &mut self,
        fragments: Array<Option<Gd<GemFragment>>>,
        check_valid: impl Fn(Gd<GemFragment>) -> bool,
        issue: GemIssue,
    ) {
        godot_print!("Gem::validate_fragments");

        fragments.iter_shared().enumerate().for_each(
            |(idx, frag_opt): (usize, Option<Gd<GemFragment>>)| {
                godot_print!("Gem::validate_fragment::for_each");
                if let Some(frag_ref) = frag_opt {
                    if !check_valid(frag_ref.clone()) {
                        godot_print!("Fragment type is not valid");
                        self.add_issue(frag_ref.bind().id().to_string(), &issue);
                    }
                } else {
                    godot_print!("Fragment is None");
                    self.add_issue(format!("Frag at idx {}.", idx), &issue);
                }
            },
        );
    }

    fn add_issues_from_link_validation(
        &mut self,
        result: &Result<(), Array<Option<Gd<GemFragmentLink>>>>,
        issue: GemIssue,
    ) {
        result.clone().err().map(|links| {
            links.iter_shared().enumerate().for_each(|(idx, link_opt)| {
                let name = if let Some(link_ref) = link_opt {
                    link_ref.get_name().to_string()
                } else {
                    format!("Frag at idx {}.", idx)
                };
                self.add_issue(name, &issue);
            });
        });
    }

    fn add_issue(&mut self, id: String, issue: &GemIssue) {
        self.issues
            .push(format!("{id} {:#?}: {}", issue.kind, issue.msg).into());
        self.issues = self.issues.clone();
    }

    pub fn observe_subresources(&self) {
        let handle_data_changed = self.base().callable("handle_data_changed");
        self.get_subresources().iter().for_each(|subresource_ref| {
                if !subresource_ref.is_connected("prop_changed".into(), handle_data_changed.clone()) {
                    let mut tmp_subresource_ref = subresource_ref.clone();
                    tmp_subresource_ref.connect("prop_changed".into(), handle_data_changed.clone());
                }
        });
    }

    pub fn process(&self, node: Gd<Node>, delta: f64) {
        self.get_fragments().iter().for_each(|(_id, frag_opt)| {
            if let Some(frag_ref) = frag_opt {
                let mut tmp_frag_ref = frag_ref.clone();
                let mut frag = tmp_frag_ref.bind_mut();
                let inputs: HashMap<String, String> = HashMap::new();
                //frag.process(node.clone(), inputs, delta)
            }
        })
    }
}
