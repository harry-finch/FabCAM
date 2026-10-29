import type { GenerateRequest } from '../api/types';
/** Test-only explicit re-authoring of public synthetic inputs, not a project reader. */
export function authorV10(request: GenerateRequest): GenerateRequest {
  request.project.cutting_version = 10;
  for (const op of request.project.operations) {
    if (
      op.kind.type === 'profile' ||
      (op.kind.type === 'pocket' &&
        !(typeof op.kind.strategy === 'object' && op.kind.strategy.kind === 'halfpipe'))
    ) {
      const tool = request.project.tools.find((t) => t.id === op.tool_id)!;
      const p = op.params;
      p.stages = {
        contract: 'main_finishing_v1',
        main: {
          tool_id: op.tool_id,
          rpm: 12000,
          feed_mm_min: 800,
          plunge_mm_min: 100,
          depth_per_pass_mm: 1,
        },
        finishing: null,
      };
      delete p.cutting;
      delete p.step;
      delete p.finish_step;
      delete p.depth_list;
      op.finish_tool_id = null;
      if (!tool) throw new Error('fixture tool');
    }
  }
  return request;
}
