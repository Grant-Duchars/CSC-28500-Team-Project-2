#include <iostream>
#include <vector>
#include <ctime>
#include <queue>

using namespace std;

void printBFS(vector<vector<int> > v, int sv, vector<bool> &visited){
    int n = v.size();
     queue<int> q;                                               //Maintaining a queue to print the vertices
     q.push(sv);
     visited[sv] = true;

     while(!q.empty()){
        int cv = q.front();
        q.pop();
        cout<<cv<<endl;
        for(int k=0;k<n;k++){
            if(v[cv][k] && !visited[k]){
                q.push(k);
                visited[k] = true;
            }
        }
     }
}

void BFS(vector<vector<int> > matrix){
    int n = matrix.size();
    vector<bool> visited(n,false);                              //Maintaining a visited array 
    int count = 0;
    for(int l=0;l<n;l++){
        if(!visited[l]){
            count++;
            printBFS(matrix,l,visited);
        }
    }
    cout<<"No of connected components is "<<count << "\n"<<endl;            
}

int main(int argc, char **argv)
{
    srand(time(NULL));                                             //seed for rand function
    for (int j = 0; j < 10; j++) {                                 //Loop to generate 10 graphs
        int vertex,edge;
        
        edge = rand()%191;                                         //Generating random edges so that each graph is equally likely to be generated
        vertex = 20;
        
        vector<vector<int>> matrix(vertex, vector<int> (vertex,0));
            for (int i = 0; i < edge; i++) {
                int fv,sv;
                fv = rand()%vertex;
                sv = rand()%vertex;
                
                while (fv == sv) {                                 //Checking for loops
                    fv = rand()%vertex;
                }
                
                matrix[fv][sv] = 1;                                //Filling in the matrix. 1 represents an edge between first vertex(fv) and second vertex(sv)
                matrix[sv][fv] = 1;
            }
            
            //Pre populated matrix to show the algorithm works
            vector<vector<int>> matrix23={{0,0,0,0,0,0,0,0,0},                          //0,1 not connected         - components 1 and 2
                                          {0,0,0,0,0,0,0,0,0},
                                          {0,0,0,0,0,0,1,0,0},      //2,6               //2 connected to 6          - component 3
                                          {0,0,0,0,1,0,0,0,0},      //3,4               //3 connected to 4          - component 4
                                          {0,0,0,1,0,1,0,0,0},      //4,3  4,5          //4 connected to 3 and 5
                                          {0,0,0,0,1,0,0,0,0},      //5,4 
                                          {0,0,1,0,0,0,0,0,1},      //6,2  6,8          //6 connected to 8
                                          {0,0,0,0,0,0,0,0,0},                          //7 not connected           - component 5
                                          {0,0,0,0,0,0,1,0,0}};     //8,6
        BFS(matrix);
        //Now the graph is ready;
    }
return 0;
}
