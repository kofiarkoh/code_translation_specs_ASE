#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DEBUG 0
#define ADD 0
#define GETSUM 1

typedef struct Node Node;

struct Node {
    int index;
    int weight;
    int child_count;
    Node* sibling;
    Node* left_most_child;
};

Node* node_list[100000];
int* Eular_Tour;
int Eular_Enter[100000];
int Eular_Exit[100000];
int Eular_Touring_Counter;


int N; // Big N is to record the given n from input // it means how much tree node.
int Q; // Big Q is to record the given q from input // it means how much query actions. 

int RQ_n;
int RQ_q;

void _init_global_var();
Node* create_Node(int index);
Node* get_Node_address(int index);
void print_Node_tree(void);
void _print_Node_tree(Node* tmp);

void Eular_Touring(void);
void _Eular_Touring();

void RQ_initRSumQ(int n_);
void RQ_update(int k, int a);
int RQ_query(int a , int b, int k, int l, int r);

void update_weight(int index, int weight);
void query_weight_sum(int target);

int main (){
    _init_global_var();
    Node* tmp = NULL;
    int child_index = 0;
    int child_count = 0;

    scanf("%d",&N);
    if(DEBUG)printf("N = %d\n",N);

    RQ_n = (N-1)*2; // for a N node tree, there's N-1 edge if there's no parallel loop or self loop or other kind of loop.

    for(int i = 0; i < N; i ++){
        tmp = NULL;
        tmp = get_Node_address(i);

        scanf("%d",&child_count);
        tmp->child_count = child_count;
        if(DEBUG)printf("Node %3d have %5d childs.\n",i,tmp->child_count);

        for(int j = 0; j< child_count; j++){
            scanf("%d",&child_index);
            if(j == 0){
                tmp->left_most_child = get_Node_address(child_index);
                tmp = tmp->left_most_child;
                if(DEBUG)printf("Node %3d have Frist child %d\n",i,child_index);
            }else{
                while(tmp->sibling != NULL){ //insurance, without this it should work//
                    tmp = tmp->sibling;
                    if(DEBUG)printf("Steping to last sibling\n");
                }
                tmp->sibling = get_Node_address(child_index);
                if(DEBUG)printf("Node %3d set %4dth child %d\n",i,j+1,child_index);
            }
        }
    }
    if(DEBUG)printf("\nFinish setup Tree\n");
    if(DEBUG)print_Node_tree();

    Eular_Touring();

    scanf("%d",&Q);
    int cmd = 0;
    int v = 0;
    int w = 0;
    int u = 0;
    while(Q>0){
        scanf("%d",&cmd);
        if(cmd == ADD){
            scanf("%d %d",&v,&w);
            update_weight(v,w);
        }

        if(cmd == GETSUM){
            scanf("%d",&u);
            query_weight_sum(u);
        }
        Q--;
    }



    return 0;
}

void _init_global_var(){
    N = 0;
    Q = 0;
    for(int i = 0; i < 100000; i++){
        node_list[i] = NULL;
        Eular_Enter[i] = -1;
        Eular_Exit[i] = -1;
    }
    Eular_Tour = NULL;
    Eular_Touring_Counter = 0;
}

Node* create_Node(int index){
    Node* tmp = malloc(sizeof(Node));
    tmp->index = index;
    tmp->weight = 0;
    tmp->child_count = 0;
    tmp->sibling = NULL;
    tmp->left_most_child = NULL;
    node_list[index] = tmp;
    return tmp;
}

Node* get_Node_address(int index){
    Node* tmp;
    if(node_list[index] == NULL){
        tmp = create_Node(index);
    }else {
        tmp = node_list[index];
    }
    return tmp;
}

void print_Node_tree(void){
    printf("*******************************************************\nStart_Print_Tree_in_Left_first_DFS\n");
    _print_Node_tree(node_list[0]);
    printf("\nEnd Tree Printing\n*******************************************************\n");
}

void _print_Node_tree(Node* tmp){
    if(tmp == NULL){
        return;
    }
    
    Node* child = NULL;

    printf("%3d ",tmp->index);

    if(tmp->left_most_child != NULL){
        child = tmp->left_most_child;
        for(int i = 0; i < tmp->child_count; i++){
            _print_Node_tree(child);
            child = child->sibling;
        }
    }

    printf("%3d ",(-1)*tmp->index);
}

void Eular_Touring(void){// this will transfer the node list to euler tour list
    if(DEBUG)printf("*******************************************************\nStart_Eular_Touring.\n");
    RQ_initRSumQ(RQ_n);
    Eular_Touring_Counter = 0;

    _Eular_Touring(node_list[0]);
    if(DEBUG)printf("\nEnd_Eular_Touring.\n*******************************************************\n");
    if(DEBUG){
        printf("   ID : ");
        for(int i = 0; i < N;i++){
            printf("%4d",i);
        }
        printf("\n");

        printf("ENTER : ");
        for(int i = 0; i < N;i++){
            printf("%4d",Eular_Enter[i]);
        }
        printf("\n");

        printf(" EXIT : ");
        for(int i  = 0; i < N;i++){
            printf("%4d",Eular_Exit[i]);
        }
        printf("\n");

        printf("*******************************************************\n");
    }
}

void _Eular_Touring(Node* tmp){
    if(tmp == NULL){
        return;
    }
    Node* child = NULL;
    if(DEBUG)printf("%3d:%3d    ",Eular_Touring_Counter,tmp->index);
    Eular_Enter[tmp->index] = Eular_Touring_Counter;
    Eular_Touring_Counter ++;
    if(tmp->left_most_child != NULL){
        child = tmp->left_most_child;
        for(int i = 0; i < tmp->child_count; i++){
            _Eular_Touring(child);
            child = child->sibling;
        }
    }
    if(tmp->index != 0){
        if(DEBUG)printf("%3d:%3d    ",Eular_Touring_Counter,(-1)*tmp->index);
        Eular_Exit[tmp->index] = Eular_Touring_Counter;
        Eular_Touring_Counter ++;
    }
}

void RQ_initRSumQ(int n_){

    RQ_n = 1;
    while(RQ_n < n_){
        RQ_n *= 2;
    }

    Eular_Tour = malloc(sizeof(int)* (2*RQ_n-1));
    if(Eular_Tour == NULL){
        printf("Init Array Failed, Out of memory or size is wrong\n");
		return;
    }

    for(int i = 0; i < 2*RQ_n-1 ; i++){
        Eular_Tour[i] = 0;
    }
    return;
}

void RQ_update(int k, int a){
    k +=RQ_n-1;
    Eular_Tour[k] += a;
    while(k > 0){
        k = (k-1)/2;
        Eular_Tour[k] = Eular_Tour[k*2+1] + Eular_Tour[k*2+2];
    }
}

int RQ_findSum(int a, int b){
    return RQ_query(a , b+1, 0, 0, RQ_n);
}

int RQ_query(int a, int b, int k, int l , int r){

	if(r <= a || b <= l){
		return 0;
	}
	if(a <= l && r <= b){
		return Eular_Tour[k];
	}
	int vl = RQ_query(a, b, k*2+1, l , (l+r)/2);
	int vr = RQ_query(a, b, k*2+2, (l+r)/2, r);
	return vl + vr; 
}

void update_weight(int index, int weight){
    RQ_update(Eular_Enter[index]-1,weight);
    RQ_update(Eular_Exit[index]-1,(-1)*weight);
}

void query_weight_sum(int target){
    printf("%d\n",RQ_findSum(0,Eular_Enter[target]-1));
}
