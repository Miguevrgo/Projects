#include "Corrector.h"
#include "Dictionary.h"
#include <QApplication>
#include <QComboBox>
#include <QHBoxLayout>
#include <QIcon>
#include <QLabel>
#include <QLineEdit>
#include <QPalette>
#include <QPushButton>
#include <QString>
#include <QTextEdit>
#include <QVBoxLayout>
#include <QWidget>

class SpellCheckerWidget : public QWidget {
  public:
    SpellCheckerWidget() {
        QPalette darkPalette;
        darkPalette.setColor(QPalette::Window, QColor(53, 53, 53));
        darkPalette.setColor(QPalette::WindowText, Qt::white);
        darkPalette.setColor(QPalette::Base, QColor(25, 25, 25));
        darkPalette.setColor(QPalette::AlternateBase, QColor(53, 53, 53));
        darkPalette.setColor(QPalette::ToolTipBase, Qt::white);
        darkPalette.setColor(QPalette::ToolTipText, Qt::white);
        darkPalette.setColor(QPalette::Text, Qt::white);
        darkPalette.setColor(QPalette::Button, QColor(53, 53, 53));
        darkPalette.setColor(QPalette::ButtonText, Qt::white);
        darkPalette.setColor(QPalette::BrightText, Qt::red);

        darkPalette.setColor(QPalette::Highlight, QColor(142, 45, 197).lighter());
        darkPalette.setColor(QPalette::HighlightedText, Qt::black);

        qApp->setPalette(darkPalette);

        QVBoxLayout *mainLayout = new QVBoxLayout(this);

        QLabel *titleLabel = new QLabel("Corrector Ortográfico");
        titleLabel->setAlignment(Qt::AlignCenter);
        QFont titleFont = titleLabel->font();
        titleFont.setPointSize(18);
        titleFont.setBold(true);
        titleLabel->setFont(titleFont);
        mainLayout->addWidget(titleLabel);

        QHBoxLayout *languageLayout = new QHBoxLayout();
        QLabel *languageLabel = new QLabel("Elige un idioma:");
        QComboBox *languageComboBox = new QComboBox();
        languageComboBox->addItem(QIcon(":/icons/english.png"), "English");
        languageComboBox->addItem(QIcon(":/icons/spanish.png"), "Spanish");
        languageLayout->addWidget(languageLabel);
        languageLayout->addWidget(languageComboBox);
        mainLayout->addLayout(languageLayout);

        QLabel *inputLabel = new QLabel("Ingresa una palabra o frase:");
        mainLayout->addWidget(inputLabel);
        QTextEdit *inputTextEdit = new QTextEdit();
        mainLayout->addWidget(inputTextEdit);

        QPushButton *correctButton = new QPushButton(QIcon(":/icons/check.png"), "Corregir");
        correctButton->setStyleSheet("padding: 10px; font-size: 16px;");
        mainLayout->addWidget(correctButton);

        connect(correctButton, &QPushButton::clicked, [=]() {
            std::string resourcedir = "../resources";
            std::string language =
                (languageComboBox->currentIndex() == 0) ? "english.txt" : "spanish.txt";

            Dictionary dictionary(resourcedir + "/" + language, language);
            Corrector corrector(dictionary);

            QString inputText = inputTextEdit->toPlainText();
            std::istringstream iss(inputText.toStdString());
            std::string word;
            inputTextEdit->clear();

            while (iss >> word) {
                std::vector<std::string> suggestions =
                    corrector.GetTopSuggestions(corrector.SuggestCorrections(word), 1);
                QString correctedWord = (!suggestions.empty())
                                            ? QString::fromStdString(suggestions[0])
                                            : QString::fromStdString(word);

                if (correctedWord == QString::fromStdString(word)) {
                    inputTextEdit->setTextColor(Qt::green);
                } else {
                    inputTextEdit->setTextColor(Qt::red);
                }

                inputTextEdit->insertPlainText(correctedWord + " ");
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
