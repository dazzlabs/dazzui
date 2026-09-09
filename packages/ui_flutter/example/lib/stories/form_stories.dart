import 'package:dazzui/dazzui.dart';

import '../storybook/gallery.dart';
import '../storybook/story.dart';

/// `Components/Form` — the run of fields, and the errors above them.
class FormStories {
  FormStories._();

  static StoryPage get page => StoryPage('Form', [
    Story('Overview', (context) => const _Overview(), layout: StoryLayout.page),
    Story('Bare', (context) => const _Bare()),
    Story('With fieldsets', (context) => const _Fieldsets()),
    Story('Server errors', (context) => const _ServerErrors()),
  ]);
}

class _Bare extends StatelessWidget {
  const _Bare();

  @override
  Widget build(BuildContext context) {
    return const SizedBox(
      width: 352,
      child: Form(
        children: [
          FormField(
            label: 'Project name',
            name: 'name',
            child: TextField(placeholder: 'Acme Web'),
          ),
          FormField(
            label: 'Description',
            name: 'description',
            child: TextField(maxLines: 3),
          ),
        ],
      ),
    );
  }
}

class _Fieldsets extends StatelessWidget {
  const _Fieldsets();

  @override
  Widget build(BuildContext context) {
    return const SizedBox(
      width: 384,
      child: Form(
        children: [
          Fieldset(
            legend: 'Billing address',
            children: [
              FormField(label: 'Street', name: 'street', child: TextField()),
              FormField(label: 'City', name: 'city', child: TextField()),
              FormField(
                label: 'Postcode',
                name: 'postcode',
                child: TextField(),
              ),
            ],
          ),
          Fieldset(
            legend: 'Delivery',
            children: [
              _Delivery(),
            ],
          ),
        ],
      ),
    );
  }
}

class _Delivery extends StatefulWidget {
  const _Delivery();

  @override
  State<_Delivery> createState() => _DeliveryState();
}

class _DeliveryState extends State<_Delivery> {
  String? _how = 'standard';

  @override
  Widget build(BuildContext context) {
    return RadioGroup<String>(
      value: _how,
      onChanged: (value) => setState(() => _how = value),
      options: const [
        RadioItem(value: 'standard', label: Text('Standard, 3–5 days')),
        RadioItem(value: 'express', label: Text('Express, next day')),
      ],
    );
  }
}

class _ServerErrors extends StatefulWidget {
  const _ServerErrors();

  @override
  State<_ServerErrors> createState() => _ServerErrorsState();
}

class _ServerErrorsState extends State<_ServerErrors> {
  Map<String, String> _errors = const {};
  String _email = 'ada@example.com';

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 352,
      child: Form(
        errors: _errors,
        children: [
          FormField(
            label: 'Email',
            name: 'email',
            hint: 'Try ada@example.com',
            child: TextField(
              placeholder: 'you@example.com',
              controller: TextEditingController(text: _email)
                ..selection = TextSelection.collapsed(offset: _email.length),
              onChanged: (value) => _email = value,
            ),
          ),
          Button(
            variant: ButtonVariant.filled,
            onPressed: () => setState(() {
              _errors = _email == 'ada@example.com'
                  ? const {'email': 'That address is already registered'}
                  : const {};
            }),
            child: const Text('Sign up'),
          ),
        ],
      ),
    );
  }
}

class _Overview extends StatelessWidget {
  const _Overview();

  @override
  Widget build(BuildContext context) {
    return const DocPage(
      'Form',
      description:
          'The run of fields, and the one thing above them that knows about '
          'all of them: errors. That is what it is for. A column of FormFields '
          'already lays out; what it cannot do is take what the server said '
          'back and put each message under the field it belongs to. A field '
          'inside a form needs only its name.',
      children: [
        DocSection(
          'Server errors, distributed',
          description:
              'Submit with the address below and the form puts the message on '
              'the field rather than at the top of the page. That is the '
              'difference between an error a reader can act on and a banner '
              'they have to translate into a field.',
          children: [_ServerErrors()],
        ),
        DocSection(
          'Fieldset',
          description:
              'A run of fields that are one question between them. The legend '
              'is the group’s accessible name, which is the whole point: '
              'a screen reader announces it before each field inside, so '
              '“Street” is heard as “Billing address, Street”. '
              'A heading in a Text above the fields looks the same and says '
              'none of that.',
          children: [_Fieldsets()],
        ),
        DocSection(
          'What it is not',
          description:
              'A fieldset claims the fields inside it belong together. Heading '
              'a run of unrelated fields is a SectionLabel’s job, or a '
              'PreferenceSection’s — reaching for a fieldset there tells a '
              'screen reader something untrue.',
          children: [_Bare()],
        ),
      ],
    );
  }
}
