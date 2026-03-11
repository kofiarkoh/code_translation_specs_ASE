#include<stdio.h>
#include<stdlib.h>

int num=0;

typedef struct node{
	int data;
	struct node *left;
	struct node *right;
}node;


node * makenode(int data){
	node *new=malloc(sizeof(node));
	new->data=data;
	new->left=NULL;
	new->right=NULL;
	return new;
}


void insert(node **p,int data){
	
	while((*p)!=NULL){
	
		if((*p)->data == data)return;
		
		if((*p)->data > data)p=&((*p)->left);
		else p=&((*p)->right);
		
	}
	
	node *new=makenode(data);
	
	(*p)=new;
	
	num++;
	
	return;
	
}

int search(node *p,int data){
	
	while(p!=NULL){
		if(p->data == data)return 1;
		
		if(p->data > data)p=p->left;
		else p=p->right;
	}
	
	return 0;
}


int main(){
	int i,q,com,x;
	node *root=NULL;
	
	scanf("%d",&q);
	
	for(i=0;i<q;i++){
		scanf("%d %d",&com,&x);
		if(com==0){
			insert(&root,x);
			printf("%d\n",num);
		}
		else {
			printf("%d\n",search(root,x));
		}
	}
	
	return 0;

}
	
