#include<stdio.h>
#include<stdlib.h>
#define N 1000

typedef struct cell{
	int data;
	struct cell *next;
}cell;

typedef struct List{
	cell *front;
	cell *rear;
}List;

List L[N];

void dump(int t){
	cell *p=L[t].front;
	
	if(p==NULL)printf("\n");
	else {
		while(p->next!=NULL){
			printf("%d ",p->data);
			p=p->next;
		}
		printf("%d\n",p->data);
	}
	
	return;
}


void insert(int t,int x){
	cell *new=malloc(sizeof(cell));
	new->data=x; new->next=NULL;
	
	if(L[t].front==NULL){
		L[t].front=new;
		L[t].rear=new;
	}
	
	else {
		L[t].rear->next=new;
		L[t].rear=new;
	}
	
	return;	
}


int main(){
	int n,q,com,i,x,y;

	
	scanf("%d %d",&n,&q);
	
	for(i=0;i<n;i++){
		L[i].front=NULL;
		L[i].rear=NULL;
	}
	
	for(i=0;i<q;i++){
		scanf("%d",&com);
		
		if(com==0){
			scanf("%d %d",&x,&y);
			insert(x,y);
		}
		
		if(com==1){
			scanf("%d",&x);
			dump(x);
		}
		
		if(com==2){
			scanf("%d %d",&x,&y);
			L[y].rear->next=L[x].front;
			L[y].rear=L[x].rear;
			L[x].front=NULL;
			L[x].rear=NULL;
		}
	}
	
	return 0;
	
}
