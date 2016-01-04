#include <iostream>
#include <string>
#include <array>
#include <vector>
#include <stdexcept>
#include <fstream>
#include <algorithm>
#include <sstream>


typedef struct Raw_graph{
    int lsize, csize;
    char start_point;
    std::vector<std::string> graph;
}raw_graph;

class node {
public:
    std::vector<int> arrive;
    std::vector<int> leave;
    char id;
    node(){id=' ';}
    node (char id_input){
        id=id_input;
    }

};


raw_graph read_file( std::string name){

    raw_graph new_graph;
    std::ifstream infile(name);
    std::string line;
    std::getline(infile, line);
    std::stringstream ss(line);
    ss >> new_graph.lsize >> new_graph.csize;
    std::getline(infile, line);
    std::stringstream sss(line);
    sss >> new_graph.start_point;
    new_graph.graph.reserve(new_graph.csize);

    int i=0;
    while (std::getline(infile, line)){
        new_graph.graph[i]=line;
    }

    return new_graph;

}

std::vector<node> process_raw(){
    raw_graph graph = read_file("input.txt");
    std::vector<node> full_graph;
    for (auto x: graph.graph){
        if (x[0] > 'A' && x[0] < 'Z'){ //if is a line with letters
            //parse string:
            for (i=0; i<graph.graph[0].size;i++){
                if (i == ' ')
                    continue;

                else if(i == '<' || i == '>' || i == '-'){
                    //TODO::Ligaçao do graph
                    switch (i) {
                        case '<':
                        case '>':
                        case '-':
                    }

                }

                else{
                    full_graph.push_back(node(i));
                }
            }
        }
        else { //It is a line with links only

        }
    }

return full_graph;
}


//TODO create the graph from raw data
//TODO study chinese postman algorithm

int main(){

}
