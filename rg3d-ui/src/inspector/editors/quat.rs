use crate::{
    core::{
        algebra::{UnitQuaternion, Vector3},
        math::UnitQuaternionExt,
        math::{quat_from_euler, RotationOrder},
    },
    inspector::{
        editors::{
            Layout, PropertyEditorBuildContext, PropertyEditorDefinition, PropertyEditorInstance,
            PropertyEditorMessageContext,
        },
        InspectorError,
    },
    message::{
        FieldKind, MessageDirection, PropertyChanged, UiMessage, UiMessageData, Vec3EditorMessage,
    },
    vec::vec3::Vec3EditorBuilder,
    widget::WidgetBuilder,
    Thickness,
};
use std::any::TypeId;

#[derive(Debug)]
pub struct QuatPropertyEditorDefinition;

impl PropertyEditorDefinition for QuatPropertyEditorDefinition {
    fn value_type_id(&self) -> TypeId {
        TypeId::of::<UnitQuaternion<f32>>()
    }

    fn create_instance(
        &self,
        ctx: PropertyEditorBuildContext,
    ) -> Result<PropertyEditorInstance, InspectorError> {
        let value = ctx.property_info.cast_value::<UnitQuaternion<f32>>()?;
        let euler = value.to_euler();
        Ok(PropertyEditorInstance {
            title: Default::default(),
            editor: Vec3EditorBuilder::new(
                WidgetBuilder::new().with_margin(Thickness::uniform(1.0)),
            )
            .with_value(Vector3::new(
                euler.x.to_degrees(),
                euler.y.to_degrees(),
                euler.z.to_degrees(),
            ))
            .build(ctx.build_context),
        })
    }

    fn create_message(
        &self,
        ctx: PropertyEditorMessageContext,
    ) -> Result<Option<UiMessage>, InspectorError> {
        let value = ctx.property_info.cast_value::<UnitQuaternion<f32>>()?;
        let euler = value.to_euler();
        let euler_degrees = Vector3::new(
            euler.x.to_degrees(),
            euler.y.to_degrees(),
            euler.z.to_degrees(),
        );
        Ok(Some(Vec3EditorMessage::value(
            ctx.instance,
            MessageDirection::ToWidget,
            euler_degrees,
        )))
    }

    fn translate_message(
        &self,
        name: &str,
        owner_type_id: TypeId,
        message: &UiMessage,
    ) -> Option<PropertyChanged> {
        if message.direction() == MessageDirection::FromWidget {
            if let UiMessageData::Vec3Editor(Vec3EditorMessage::Value(value)) = message.data() {
                let euler = Vector3::new(
                    value.x.to_radians(),
                    value.y.to_radians(),
                    value.z.to_radians(),
                );
                let rotation = quat_from_euler(euler, RotationOrder::XYZ);
                return Some(PropertyChanged {
                    owner_type_id,
                    name: name.to_string(),
                    value: FieldKind::object(rotation),
                });
            }
        }
        None
    }

    fn layout(&self) -> Layout {
        Layout::Horizontal
    }
}
