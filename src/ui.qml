import QtQuick 2.0
import QtQuick.Dialogs 1.0
import QtQuick.Controls 2.0
import QtQuick.Window 2.0
import UI 1.0

Window {
    id: window
    width: 200
    height: 200
    visible: true
    title: qsTr("Movie Report Generator")

    UI {
        id: model
    }

    Rectangle {
        Button {
            text: "Create Report"
            onClicked: inFile.open()
        }
    }

    FileDialog {
        id: inFile
        title: "Input CSV File"
        folder: shortcuts.home
        selectExisting: true
        defaultSuffix: "csv"
        onAccepted: {
            model.open(inFile.fileUrl)
            outFile.open()
        }
    }

    FileDialog {
        id: outFile
        title: "Save Report"
        folder: shortcuts.home
        selectExisting: false
        defaultSuffix: "html"
        onAccepted: {
            model.save(outFile.fileUrl)
            Qt.quit()
        }
    }
}