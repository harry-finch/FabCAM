import { project } from './project.svelte';
import { loadConfiguredSources, type LoadedSource } from './feeds-speeds-sources';
class RecommendationSources {
  rows = $state<LoadedSource[]>([]);
  error = $state('');
  loading = $state(false);
  private key = '';
  private reference: Promise<string> | undefined;
  async refresh() {
    const settings = project.data.settings.feedsSpeedsSources;
    const key = JSON.stringify(settings);
    if (key === this.key) return;
    this.key = key;
    this.loading = true;
    this.error = '';
    this.rows = [];
    try {
      const rows = await loadConfiguredSources(settings, () => {
        return (this.reference ??= fetch(
          `${import.meta.env.BASE_URL}feeds-speeds/sienci-router-reference.v1.json`,
        ).then((r) => {
          if (!r.ok) throw new Error('Reference source unavailable');
          return r.text();
        }));
      });
      if (key === this.key) this.rows = rows;
    } catch (e) {
      if (key === this.key) {
        this.error = String(e);
        this.key = '';
        this.reference = undefined;
      }
    } finally {
      if (key === this.key || this.key === '') this.loading = false;
    }
  }
}
export const recommendationSources = new RecommendationSources();
