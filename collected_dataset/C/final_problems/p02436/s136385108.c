#include <stdio.h>
#include <stdlib.h>

#define xMAX 1000000000

typedef struct _list{
    int data;
    struct _list *next;
}LIST;
typedef struct _que{
    LIST *head;
    LIST *tail;
}QUE;

int main(){
    int i;
    int n,q;
    scanf("%d %d\n",&n,&q);
    
    QUE Q[n];
    for(i=0;i<n;i++){
        Q[i].head=NULL;
        Q[i].tail=NULL;
    }
    
    LIST *p;
    int com,t,x;
    while(q--){
        scanf("%d %d \n",&com,&t);
        switch(com){
            case 0:
                //pushback
                scanf("%d\n",&x);
                if(Q[t].head==NULL){
                    Q[t].head=(LIST*)malloc(sizeof(LIST));
                    Q[t].head->data=x;
                    Q[t].head->next=NULL;
                    Q[t].tail=Q[t].head;
                }else{
                    Q[t].tail->next=(LIST*)malloc(sizeof(LIST));
                    Q[t].tail=Q[t].tail->next;
                    Q[t].tail->data=x;
                    Q[t].tail->next=NULL;
                }
                break;
            case 1:
                //dump
                if(Q[t].head!=NULL)printf("%d\n",Q[t].head->data);
                break;
            case 2:
                //clear
                if(Q[t].head==NULL)break;
                
                p=Q[t].head;
                if(Q[t].head->next!=NULL){
                    Q[t].head=Q[t].head->next;
                }else{
                    Q[t].head=NULL;
                    Q[t].tail=NULL;
                }
                free(p);
                break;
        }
    }
    
    LIST *tmp;
    for(i=0;i<n;i++){
        if(Q[i].head!=NULL){
            p=Q[i].head;
            while(p!=NULL){
                tmp=p;
                free(p);
                p=tmp->next;
            }
        }
    }
    
    return 0;
}
