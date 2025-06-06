# Copyright (c) 2023 VEXXHOST, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may
# not use this file except in compliance with the License. You may obtain
# a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
# WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
# License for the specific language governing permissions and limitations
# under the License.

from magnum import objects

from magnum_cluster_api.integrations import common


def is_enabled(cluster: objects.Cluster) -> bool:
    return common.is_enabled(
        cluster, "manila_csi_enabled", "sharev2"
    ) or common.is_enabled(cluster, "manila_csi_enabled", "share")


def get_image(cluster: objects.Cluster) -> str:
    return common.get_cloud_provider_image(
        cluster, "manila_csi_plugin_tag", "manila-csi-plugin"
    )
