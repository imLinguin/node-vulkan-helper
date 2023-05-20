declare module "vulkan-helper" {
  export function get_instance_version(): [
    maj: number,
    min: number,
    patch: number
  ];
  export function get_physical_versions(): {
    title: string;
    major: number;
    minor: number;
    patch: number;
  }[];
}
