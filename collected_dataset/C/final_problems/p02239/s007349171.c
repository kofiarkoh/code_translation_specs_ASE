/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   main.c
 * Author: kundaikwangwari
 *
 * Created on July 23, 2019, 8:15 PM
 */

#include <stdio.h>
#include <stdlib.h>
#include <limits.h>
#define NIL -1
#define White 0
#define Grey 1
#define Black 2
#define size 105

/*
 * 
 */
int Timetrack,n,head,tail,Que[size];
int table[size][size], Distance[size];

void insert(int val){
    
    Que[tail++]=val;
}

int Retrieve(){
    return Que[head++];
}


void BreathFirstSearch(int val, int *Color){
   
    for(int i = 0; i < n + 1; i++){
        Distance[i] = INT_MAX;
        Color[i] = White;
        }
    Distance[val] = 0;
    Color[val] = Grey;
    insert(val);
 
    while(head!=tail){
  int node =Retrieve();

    //printf("going intp node %d, color %d\n",V,Color[V]);
    
    for(int u = 1; u < n + 1; u++){
        if(table[node][u] == NIL)continue;
        if(Color[u] == White){
    
    Distance[u] = Distance[node]+1;
    insert(u);
    Color[u] = Black;
            }
        }
    }
    }

int main(int argc, char** argv) {

    int V,k,edge;
    scanf("%d",&n);
    
    int Color[n+1];
    for(int i = 1; i < n + 1; i++){
        for(int a = 1; a < n + 1; a++)table[i][a] = NIL;
}
    
    for(int i = 0; i < n ; i++){
        
        scanf("%d%d",&V,&k);
        //printf("V %d edges %d\n",V,edge);
        for(int a = 0; a < k; a++){
            scanf("%d",&edge);
            table[V][edge] = 1;
            //printf("table(%d)(%d) has value %d \n",V,edge,table[V][edge]);
        }
    }
    
    
        BreathFirstSearch(1,Color);
        for(int i = 1; i < n + 1; i++){
            if(Distance[i] != INT_MAX)printf("%d %d\n",i,Distance[i]);
            else printf("%d -1\n",i);
        }
    
    return (0);
}


