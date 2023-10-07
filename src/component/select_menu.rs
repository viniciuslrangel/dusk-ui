use twilight_model::channel::message::component::SelectMenuOption;

pub struct SelectMenu<D> {
    phantom: std::marker::PhantomData<D>,
    pub id: String,
    pub disabled: bool,
    pub max_values: Option<u8>,
    pub min_values: Option<u8>,
    pub options: Vec<SelectMenuOption>,
    pub placeholder: Option<String>,
}