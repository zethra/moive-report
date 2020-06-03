import QtQuick 2.0
import QtQuick.Dialogs 1.0
import QtQuick.Controls 2.0
import QtQuick.Window 2.0
import UI 1.0
import QtQuick.Layouts 1.0

Window {
    id: window
    width: 500
    height: 300
    visible: true
    title: qsTr("Movie Report Generator")

    UI {
        id: model
    }

    Rectangle {
        anchors.fill: parent
        ColumnLayout {
            anchors.fill: parent

            Text {
                text: "Movie Report Generator"
                font.pointSize: 24
                Layout.alignment: Qt.AlignCenter
            }

            Button {
                text: "Create Report"
                onClicked: inFile.open()
                Layout.alignment: Qt.AlignCenter
                Layout.preferredWidth: parent.width / 3
                Layout.preferredHeight: parent.height / 5
            }
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