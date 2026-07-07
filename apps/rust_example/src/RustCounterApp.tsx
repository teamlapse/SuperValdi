import { StatefulComponent } from 'valdi_core/src/Component';
import { Style } from 'valdi_core/src/Style';
import { systemBoldFont, systemFont } from 'valdi_core/src/SystemFont';
import { Label, View } from 'valdi_tsx/src/NativeTemplateElements';

import { count$, formatCountLabel, incrementCount, rustPayloadSize } from './CounterStore';

interface CounterState {
  count: number;
}

/**
 * @ViewModel
 * @ExportModel
 */
export interface ViewModel {}

/**
 * @Context
 * @ExportModel
 */
export interface ComponentContext {}

/**
 * @Component
 * @ExportModel
 */
export class App extends StatefulComponent<ViewModel, CounterState, ComponentContext> {
  state: CounterState = {
    count: 0,
  };

  onCreate(): void {
    this.registerDisposable(
      count$.subscribe(count => {
        this.setState({ count });
      }),
    );
  }

  onRender(): void {
    <view style={styles.root}>
      <view style={styles.panel}>
        <label style={styles.eyebrow} value="Rust native module" font={systemBoldFont(13)} />
        <label style={styles.count} value={`${this.state.count}`} font={systemBoldFont(64)} />
        <label style={styles.countLabel} value={formatCountLabel(this.state.count)} font={systemFont(15)} />
        <view style={styles.button} onTap={this.incrementCounter}>
          <label style={styles.buttonLabel} value="Increment in Rust" font={systemBoldFont(17)} />
        </view>
        <label
          style={styles.caption}
          value={`The TypeScript @ExportModule declaration generates the bridge; Rust returns the observable count source and typed payload size ${rustPayloadSize}.`}
          font={systemFont(13)}
        />
      </view>
    </view>;
  }

  private readonly incrementCounter = (): void => {
    incrementCount();
  };
}

const styles = {
  root: new Style<View>({
    width: '100%',
    height: '100%',
    backgroundColor: '#f6f8fb',
    justifyContent: 'center',
    alignItems: 'center',
    padding: 24,
  }),

  panel: new Style<View>({
    width: '100%',
    maxWidth: 360,
    padding: 24,
    borderRadius: 8,
    backgroundColor: 'white',
    alignItems: 'center',
    boxShadow: '0 3 16 rgba(15, 23, 42, 0.16)',
  }),

  eyebrow: new Style<Label>({
    color: '#38506b',
    marginBottom: 12,
  }),

  count: new Style<Label>({
    color: '#111827',
    marginBottom: 8,
  }),

  countLabel: new Style<Label>({
    color: '#38506b',
    marginBottom: 20,
  }),

  button: new Style<View>({
    width: '100%',
    minHeight: 52,
    borderRadius: 8,
    backgroundColor: '#0f766e',
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: 16,
  }),

  buttonLabel: new Style<Label>({
    color: 'white',
  }),

  caption: new Style<Label>({
    color: '#5f6f82',
    textAlign: 'center',
    lineHeight: 1.35,
  }),
};
