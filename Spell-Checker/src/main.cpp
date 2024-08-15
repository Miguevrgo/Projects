#include "Corrector.h"
#include "Dictionary.h"
#include <QApplication>
#include <QComboBox>
#include <QLabel>
#include <QPushButton>
#include <QString>
#include <QTextEdit>
#include <QVBoxLayout>
#include <QWidget>

class SpellCheckerWidget : public QWidget {
  public:
    SpellCheckerWidget() {
        QVBoxLayout *layout = new QVBoxLayout(this);

        QLabel *label1 = new QLabel("Elige un idioma:");
        layout->addWidget(label1);

        QComboBox *comboBox = new QComboBox();
        comboBox->addItem("English");
        comboBox->addItem("Spanish");
        layout->addWidget(comboBox);

        QLabel *label2 = new QLabel("Ingresa una palabra o frase:");
        layout->addWidget(label2);

        QTextEdit *textEdit = new QTextEdit();
        layout->addWidget(textEdit);

        QPushButton *button = new QPushButton("Corregir");
        layout->addWidget(button);

        connect(button, &QPushButton::clicked, [=]() {
            std::string resourcedir = "../resources";
            std::string language = (comboBox->currentIndex() == 0) ? "english.txt" : "spanish.txt";

            Dictionary dictionary(resourcedir + "/" + language, language);
            Corrector corrector(dictionary);

            QString inputText = textEdit->toPlainText();
            std::istringstream iss(inputText.toStdString());
            std::string word;
            textEdit->clear();

            while (iss >> word) {
                std::vector<std::string> suggestions =
                    corrector.GetTopSuggestions(corrector.SuggestCorrections(word), 1);
                QString correctedWord = (!suggestions.empty())
                                            ? QString::fromStdString(suggestions[0])
                                            : QString::fromStdString(word);

                if (correctedWord == QString::fromStdString(word)) {
                    textEdit->setTextColor(Qt::green);
                } else {
                    textEdit->setTextColor(Qt::red);
                }

                textEdit->insertPlainText(correctedWord + " ");
            }
        });
    }
};

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);

    SpellCheckerWidget widget;
    widget.setWindowTitle("Corrector Ortográfico");
    widget.resize(500, 300);
    widget.show();

    return app.exec();
}
