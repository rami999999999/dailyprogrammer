#include <iostream>
#include <string>
#include <array>
#include <vector>
#include <stdexcept>
#include <fstream>
#include <algorithm>

using namespace std;

class Show {


public:
    int s_start;
    int s_end;
    string name = "empty";

    Show(){};
    Show(string start_input, string end_input, string name_input) {
        //cout << start_input << " || " << end_input;
        if (start_input.length() != 4 || end_input.length() != 4)
            throw std::invalid_argument("you must enter 4 digits");
        s_start = std::stoi(start_input.substr(0,2)) * 60 + std::stoi(start_input.substr(2, 2));
        s_end = std::stoi(end_input.substr(0,2)) * 60 + std::stoi(end_input.substr(2, 2));
        //cout << "start: " << s_start << " end: " << s_end << endl;
        name = name_input;
    }

    int get_duration(){
        return s_end-s_start;
    }
    static bool compare(Show a, Show b){
        return a.get_duration() < b.get_duration();
    }

};



int main() {

    std::vector<Show> list;
    std::ifstream infile("thefile.txt");
    string line;
    int j=0;



    while (std::getline(infile, line)){
        cout << line.substr(0,4) << " || " << line.substr(5,4) <<  " || " << line.substr(10) << endl;
        list.push_back(Show(line.substr(0,4),line.substr(5,4),line.substr(10)));

        }

    std::sort(list.begin(),list.end(),Show::compare);
    std::vector<Show> recorded;
    for (Show i: list){
        if (recorded.size() == 0) { recorded.push_back(i);}
        else {
            //cout << i.name << endl;
            for ( j=0; j<recorded.size(); j++) {
                if (j==0 && i.s_end < recorded[j].s_start) { recorded.insert(recorded.begin(),i); break;}
                else if (j+2 > recorded.size() && recorded[j].s_end < i.s_start) { recorded.push_back(i);break; }
                else if (i.s_start > recorded[j].s_end && i.s_end < recorded[j+1].s_end) { recorded.insert(recorded.begin()+j,i); break; }

                }
            }

    }

    cout << "# of shows: " << recorded.size() << endl;
    for (Show i: recorded) { cout << i.name << endl;}


}
