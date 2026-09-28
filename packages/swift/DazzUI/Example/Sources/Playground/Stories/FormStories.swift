import DazzUI
import SwiftUI

/// `Components/Form` — the run of fields, and the errors above them.
enum FormStories {
  static var page: StoryPage {
    StoryPage("Form") {
      Story("Overview", layout: .page) { Overview() }
      Story("Bare") { Plain() }
      Story("With fieldsets") { Grouped() }
      Story("Server errors") { ServerErrors() }
    }
  }

  struct Plain: View {
    @State private var name = "Acme Web"
    @State private var description = ""

    var body: some View {
      Form {
        FormField(label: "Project name", name: "name") { TextField(text: $name) }
        FormField(label: "Description", name: "description") {
          TextField(text: $description, multiline: true)
        }
      }
      .frame(width: 352)
    }
  }

  struct Grouped: View {
    @State private var street = ""
    @State private var city = ""
    @State private var delivery = "standard"

    var body: some View {
      Form {
        Fieldset(legend: "Billing address") {
          FormField(label: "Street", name: "street") { TextField(text: $street) }
          FormField(label: "City", name: "city") { TextField(text: $city) }
        }
        Fieldset(legend: "Delivery") {
          RadioGroup(
            options: [
              RadioGroupOption(value: "standard", label: "Standard, 3–5 days"),
              RadioGroupOption(value: "express", label: "Express, next day"),
            ],
            value: delivery,
            onValueChange: { delivery = $0 }
          )
        }
      }
      .frame(width: 384)
    }
  }

  struct ServerErrors: View {
    @State private var email = "ada@example.com"
    @State private var errors: [String: String] = [:]

    var body: some View {
      Form(errors: errors) {
        FormField(label: "Email", hint: "Try ada@example.com", name: "email") {
          TextField(text: $email, placeholder: "you@example.com")
        }
        Button("Sign up", variant: .filled) {
          errors = email == "ada@example.com"
            ? ["email": "That address is already registered"]
            : [:]
        }
      }
      .frame(width: 352)
    }
  }

  private struct Overview: View {
    var body: some View {
      Page(
        "Form",
        description:
          "The run of fields, and the one thing above them that knows about all of them: "
          + "errors. That is what it is for. A stack of FormFields already lays out; what it "
          + "cannot do is take what the server said back and put each message under the field "
          + "it belongs to. A field inside a form needs only its name."
      ) {
        Section(
          "Server errors, distributed",
          description:
            "Submit with the address below and the form puts the message on the field rather "
            + "than at the top of the page. That is the difference between an error a reader "
            + "can act on and a banner they have to translate into a field."
        ) {
          ServerErrors()
        }
        Section(
          "Fieldset",
          description:
            "A run of fields that are one question between them. The legend names the group, "
            + "which is the whole point: it is announced before each field inside, so “Street” "
            + "is heard as “Billing address, Street”."
        ) {
          Grouped()
        }
        Section(
          "What it is not",
          description:
            "A fieldset claims the fields inside it belong together. Heading a run of unrelated "
            + "fields is a SectionLabel’s job, or a PreferenceSection’s — reaching for a "
            + "fieldset there says something untrue."
        ) {
          Plain()
        }
      }
    }
  }
}
