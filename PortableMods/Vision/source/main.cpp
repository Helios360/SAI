#include <opencv2/opencv.hpp>
#include <opencv2/face.hpp>
#include <iostream>
#include <filesystem>
#include <map>

using namespace std;
using namespace cv;
using namespace cv::face;
namespace fs = std::filesystem;

// Global recognizer
Ptr<LBPHFaceRecognizer> recognizer = LBPHFaceRecognizer::create();
map<int, string> labelNames;

void detectAndDraw(Mat& img, CascadeClassifier& cascade, CascadeClassifier& nestedCascade, double scale);

void trainRecognizer(const string& dataset_path) {
    vector<Mat> images;
    vector<int> labels;

    CascadeClassifier face_cascade;
    face_cascade.load("/usr/local/share/opencv4/haarcascades/haarcascade_frontalface_default.xml");

    if (face_cascade.empty()) {
        cerr << "Failed to load face cascade!" << endl;
        exit(1);
    }

    int label = 0;
    for (const auto& person_dir : fs::directory_iterator(dataset_path)) {
        if (person_dir.is_directory()) {
            string person_name = person_dir.path().filename().string();
            labelNames[label] = person_name;

            for (const auto& image_file : fs::directory_iterator(person_dir)) {
                if (image_file.is_regular_file()) {
                    Mat img_color = imread(image_file.path().string());
                    if (img_color.empty()) continue;

                    Mat img_gray;
                    cvtColor(img_color, img_gray, COLOR_BGR2GRAY);

                    // Detect face
                    vector<Rect> faces;
                    face_cascade.detectMultiScale(img_gray, faces, 1.1, 3, 0, Size(100, 100));

                    if (!faces.empty()) {
                        // Crop the first face detected
                        Mat face = img_gray(faces[0]);

                        // Resize for consistency
                        resize(face, face, Size(100, 100));

                        // Histogram equalization
                        equalizeHist(face, face);

                        images.push_back(face);
                        labels.push_back(label);
                    }
                }
            }
            label++;
        }
    }

    if (images.empty()) {
        cerr << "No faces found in training data!" << endl;
        exit(1);
    }

    recognizer->train(images, labels);
    recognizer->save("face_model.xml");

    cout << "Training complete with " << images.size() << " face samples." << endl;
}

int main() {
    CascadeClassifier cascade, nestedCascade;
    double scale = 1.0;

    string dataset_path = "../../Hippocampus/Peoples";
    string face_cascade_path = "/usr/local/share/opencv4/haarcascades/haarcascade_frontalface_default.xml";
    string eye_cascade_path = "/usr/local/share/opencv4/haarcascades/haarcascade_eye.xml";

    if (!cascade.load(face_cascade_path) || !nestedCascade.load(eye_cascade_path)) {
        cerr << "Error loading cascades" << endl;
        return -1;
    }

    trainRecognizer(dataset_path); // Load and train data

    recognizer->read("face_model.xml"); // Load model from file

    VideoCapture capture(0);
    if (!capture.isOpened()) {
        cerr << "Could not open camera" << endl;
        return -1;
    }

    Mat frame;
    cout << "Starting Face Detection and Recognition..." << endl;
    while (true) {
        capture >> frame;
        if (frame.empty()) break;

        detectAndDraw(frame, cascade, nestedCascade, scale);

        char c = (char)waitKey(10);
        if (c == 27 || c == 'q' || c == 'Q') break;
    }
    return 0;
}

void detectAndDraw(Mat& img, CascadeClassifier& cascade, CascadeClassifier& nestedCascade, double scale) {
    vector<Rect> faces;
    Mat gray, smallImg;

    cvtColor(img, gray, COLOR_BGR2GRAY);
    resize(gray, smallImg, Size(), scale, scale, INTER_LINEAR);
    equalizeHist(smallImg, smallImg);

    cascade.detectMultiScale(smallImg, faces, 1.1, 2, 0 | CASCADE_SCALE_IMAGE, Size(30, 30));

    for (size_t i = 0; i < faces.size(); i++) {
        Rect r = faces[i];
        Mat faceROI = gray(r);
        int label;
        double confidence;

        recognizer->predict(faceROI, label, confidence);
        string text = "Unknown";

        if (labelNames.find(label) != labelNames.end()) {
            text = labelNames[label] + " (" + to_string((int)confidence) + ")";
        }

        rectangle(img, r, Scalar(255, 0, 0), 2);
        putText(img, text, Point(r.x, r.y - 10), FONT_HERSHEY_SIMPLEX, 0.9, Scalar(0, 255, 0), 2);
    }

    imshow("Face Recognition", img);
}
