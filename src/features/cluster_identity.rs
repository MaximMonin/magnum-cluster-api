use crate::{
    cluster_api::{
        clusterclasses::{
            ClusterClassPatches, ClusterClassPatchesDefinitions,
            ClusterClassPatchesDefinitionsJsonPatches,
            ClusterClassPatchesDefinitionsJsonPatchesValueFrom,
            ClusterClassPatchesDefinitionsSelector,
            ClusterClassPatchesDefinitionsSelectorMatchResources,
            ClusterClassPatchesDefinitionsSelectorMatchResourcesMachineDeploymentClass,
            ClusterClassVariables, ClusterClassVariablesSchema,
        },
        openstackclustertemplates::OpenStackClusterTemplate,
        openstackmachinetemplates::OpenStackMachineTemplate,
    },
    features::{
        ClusterClassVariablesSchemaExt, ClusterFeatureEntry, ClusterFeaturePatches,
        ClusterFeatureVariables,
    },
};
use cluster_feature_derive::ClusterFeatureValues;
use kube::CustomResourceExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ClusterFeatureValues)]
pub struct FeatureValues {
    #[serde(rename = "clusterIdentityRefName")]
    pub cluster_identity_ref_name: String,
}

pub struct Feature {}

impl ClusterFeaturePatches for Feature {
    fn patches(&self) -> Vec<ClusterClassPatches> {
        vec![ClusterClassPatches {
            name: "clusterIdentityRefName".into(),
            definitions: Some(vec![
                ClusterClassPatchesDefinitions {
                    selector: ClusterClassPatchesDefinitionsSelector {
                        api_version: OpenStackMachineTemplate::api_resource().api_version,
                        kind: OpenStackMachineTemplate::api_resource().kind,
                        match_resources: ClusterClassPatchesDefinitionsSelectorMatchResources {
                            control_plane: Some(true),
                            machine_deployment_class: Some(ClusterClassPatchesDefinitionsSelectorMatchResourcesMachineDeploymentClass {
                                names: Some(vec!["default-worker".to_string()])
                            }),
                            ..Default::default()
                        },
                    },
                    json_patches: vec![
                        ClusterClassPatchesDefinitionsJsonPatches {
                            op: "replace".into(),
                            path: "/spec/template/spec/identityRef/name".into(),
                            value_from: Some(ClusterClassPatchesDefinitionsJsonPatchesValueFrom {
                                variable: Some("clusterIdentityRefName".into()),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                    ],
                },
                ClusterClassPatchesDefinitions {
                  selector: ClusterClassPatchesDefinitionsSelector {
                      api_version: OpenStackClusterTemplate::api_resource().api_version,
                      kind: OpenStackClusterTemplate::api_resource().kind,
                      match_resources: ClusterClassPatchesDefinitionsSelectorMatchResources {
                          infrastructure_cluster: Some(true),
                          ..Default::default()
                      },
                  },
                  json_patches: vec![
                      ClusterClassPatchesDefinitionsJsonPatches {
                          op: "add".into(),
                          path: "/spec/template/spec/identityRef/name".into(),
                          value_from: Some(ClusterClassPatchesDefinitionsJsonPatchesValueFrom {
                              variable: Some("clusterIdentityRefName".into()),
                              ..Default::default()
                          }),
                          ..Default::default()
                      },
                  ],
              },
            ]),
            ..Default::default()
        }]
    }
}

inventory::submit! {
    ClusterFeatureEntry{ feature: &Feature {} }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::test::TestClusterResources;
    use crate::resources::fixtures::default_values;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_patches() {
        let feature = Feature {};

        let values = default_values();
        let patches = feature.patches();

        let mut resources = TestClusterResources::new();
        resources.apply_patches(&patches, &values);

        assert_eq!(
            resources
                .control_plane_openstack_machine_template
                .spec
                .template
                .spec
                .identity_ref
                .expect("identity ref should be set")
                .name,
            values.cluster_identity_ref_name.clone()
        );

        assert_eq!(
            resources
                .worker_openstack_machine_template
                .spec
                .template
                .spec
                .identity_ref
                .expect("identity ref should be set")
                .name,
            values.cluster_identity_ref_name.clone()
        );

        assert_eq!(
            resources
                .openstack_cluster_template
                .spec
                .template
                .spec
                .identity_ref
                .name,
            values.cluster_identity_ref_name.clone()
        );
    }
}
