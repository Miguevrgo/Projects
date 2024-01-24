#include <wx/wx.h>
#include <sstream>
#include "Corrector.h"
#include "Dictionary.h"

class SpellCheckerFrame : public wxFrame {
public:
    SpellCheckerFrame(const wxString& title)
            : wxFrame(NULL, wxID_ANY, title, wxDefaultPosition, wxSize(500, 300)) {

        wxPanel *panel = new wxPanel(this, -1);
        wxBoxSizer *vbox = new wxBoxSizer(wxVERTICAL);

        wxStaticText *st1 = new wxStaticText(panel, wxID_ANY, wxT("Elige un idioma:")); // Spanish or English
        vbox->Add(st1, 0, wxLEFT | wxTOP, 10);

        wxArrayString choices;
        choices.Add(wxT("English"));
        choices.Add(wxT("Spanish"));
        wxChoice *choice = new wxChoice(panel, wxID_ANY, wxDefaultPosition, wxDefaultSize, choices);
        vbox->Add(choice, 0, wxLEFT | wxTOP, 10);

        wxStaticText *st2 = new wxStaticText(panel, wxID_ANY, wxT("Ingresa una palabra o frase:"));
        vbox->Add(st2, 0, wxLEFT | wxTOP, 10);

        wxTextCtrl *tc = new wxTextCtrl(panel, wxID_ANY, "", wxDefaultPosition, wxDefaultSize, wxTE_MULTILINE);
        vbox->Add(tc, 1, wxEXPAND | wxLEFT | wxRIGHT | wxTOP, 10);

        wxButton *btn = new wxButton(panel, wxID_ANY, wxT("Corregir"));
        vbox->Add(btn, 0, wxALIGN_RIGHT | wxRIGHT | wxBOTTOM, 10);

        panel->SetSizer(vbox);
        Centre();

        btn->Bind(wxEVT_BUTTON, [this, tc, choice](wxCommandEvent& event) {
            std::string resourcedir = "./resources";
            std::string language = (choice->GetSelection() == 0) ? "english.txt" : "spanish.txt";

            Dictionary dictionary(resourcedir + "/" + language, language);
            Corrector corrector(dictionary);

            std::string inputText = tc->GetValue().ToStdString();
            std::istringstream iss(inputText);
            std::string word;
            wxString result;
            wxTextAttr redAttr(*wxRED); // Red text for misspelled words
            wxTextAttr greenAttr(*wxGREEN); // Green text for unchanged words

            tc->Clear();

            while (iss >> word) { // Read word or sentence
                std::vector<std::string> suggestions = corrector.GetTopSuggestions(corrector.SuggestCorrections(word), 1);
                wxString correctedWord = (!suggestions.empty()) ? wxString(suggestions[0]) : wxString(word);

                wxTextAttr colorAttr = (correctedWord.IsSameAs(word, false)) ? greenAttr : redAttr;
                tc->SetDefaultStyle(colorAttr);
                tc->AppendText(correctedWord + " ");
            }
        });
    }
};

class SpellCheckerApp : public wxApp {
public:
    virtual bool OnInit() {
        SpellCheckerFrame *frame = new SpellCheckerFrame(wxT("Corrector Ortográfico"));
        frame->Show(true);
        return true;
    }
};

wxIMPLEMENT_APP(SpellCheckerApp);
