import type { Qualification, VerificationInput, VerificationReply } from './qualification';
export interface VerificationWorker {
  onmessage: ((event: MessageEvent<VerificationReply>) => void) | null;
  onerror: ((event: ErrorEvent) => void) | null;
  onmessageerror: ((event: MessageEvent) => void) | null;
  postMessage(value: VerificationInput): void;
  terminate(): void;
}
/** One request per worker. Termination cancels blocked synchronous WASM too. */
export class QualificationRun {
  private worker: VerificationWorker | null = null;
  private sequence = 0;
  constructor(private create: () => VerificationWorker) {}
  cancel() {
    this.sequence++;
    this.worker?.terminate();
    this.worker = null;
  }
  start(input: VerificationInput, publish: (state: Qualification) => void) {
    this.cancel();
    const sequence = this.sequence;
    publish({ key: input.key, phase: 'pending' });
    const finish = (state: Qualification) => {
      if (this.sequence !== sequence) return;
      this.cancel();
      publish(state);
    };
    try {
      const worker = (this.worker = this.create());
      worker.onmessage = ({ data }) => {
        if ('error' in data) finish({ key: input.key, phase: 'failed', message: data.error });
        else if (
          data.result.key !== input.key ||
          !data.result.front ||
          (!!input.back && !data.result.back)
        )
          finish({
            key: input.key,
            phase: 'failed',
            message: 'Incomplete or mismatched verification result',
          });
        else finish({ ...data.result, phase: 'current' });
      };
      worker.onerror = (event) =>
        finish({ key: input.key, phase: 'failed', message: event.message });
      worker.onmessageerror = () =>
        finish({ key: input.key, phase: 'failed', message: 'Unreadable verification response' });
      worker.postMessage(input);
    } catch (error) {
      finish({ key: input.key, phase: 'failed', message: String(error) });
    }
  }
}
