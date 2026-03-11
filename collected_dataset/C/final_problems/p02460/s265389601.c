#include<stdio.h>
#include<stdlib.h>
#include<time.h>
#include<string.h>

typedef struct node{
	char key[25];
	int data;
	int priority;
	struct node *left;
	struct node *right;
}node;

node * Delete(node **,char[]);
node * _delete(node **,char[]);

int find(node *p,char str[]){

	while(p!=NULL){
		if(strcmp(p->key,str)==0)return p->data;
		else if(strcmp(p->key,str)>0)p=p->left;
		else p=p->right;
	}
	
	return 0;
	
}

void update(node *p,char str[],int x){

	while(p!=NULL){
		if(strcmp(p->key,str)==0){
			p->data=x;
			return;
		}
		else if(strcmp(p->key,str)>0)p=p->left;
		else p=p->right;
	}
	
	return;
	
}


node * makenode(char key[],int x,int y){
	node *new=malloc(sizeof(node));
	strcpy(new->key,key);
	new->data=x;
	new->priority=y;
	new->left=NULL;
	new->right=NULL;
	return new;
}

node *rightRotate(node *p){
	node *q=p->left;
	p->left=q->right;
	q->right=p;
	return q;
}

node *leftRotate(node *p){
	node *q=p->right;
	p->right=q->left;
	q->left=p;
	return q;
}


node* insert(node **t,char key[],int x,int priority){
	node *new;
	
	if((*t)==NULL)
		return new=makenode(key,x,priority);
	if (strcmp((*t)->key,key)==0)
		return *t;
	
	if(strcmp(key,(*t)->key)<0){
		(*t)->left=insert(&(*t)->left,key,x,priority);
		if((*t)->priority < (*t)->left->priority)
			(*t)=rightRotate((*t));
	}
	else {
		(*t)->right=insert(&(*t)->right,key,x,priority);
		if((*t)->priority < (*t)->right->priority)
			(*t)=leftRotate((*t));
	}
	
	return *t;
}

node * Delete(node **t,char key[]){
	if((*t)==NULL)
		return NULL;
		
	if(strcmp(key,(*t)->key)<0)
		(*t)->left=Delete(&((*t)->left),key);
		
	else if(strcmp(key,(*t)->key)>0)
		(*t)->right=Delete(&((*t)->right),key);
		
	else return _delete(t,key);
	
	return *t;
}

node * _delete(node **t,char key[]){
	if((*t)->left==NULL && (*t)->right==NULL)
		return NULL;
	else if((*t)->left==NULL)
		(*t)=leftRotate(*t);
	else if((*t)->right==NULL)
		(*t)=rightRotate(*t);
	else {
		if((*t)->left->priority > (*t)->right->priority)
			(*t)=rightRotate(*t);
		else
			(*t)=leftRotate(*t);
	}
	return Delete(t,key);
}

int main(){
	int q,x,com;
	char str[25];
	node *root=NULL;
	srand((unsigned)time(NULL));
	
	scanf("%d",&q);
	
	for(int i=0;i<q;i++){
		scanf("%d %s",&com,str);
		
		if(com==0){
			scanf("%d",&x);
			if(!find(root,str)){
				root=insert(&root,str,x,rand()%200001);
			}
			else {
				update(root,str,x);
			}
		}
		else if(com==1){
			printf("%d\n",find(root,str));
		}
		
		else if(com==2){
			root=Delete(&root,str);
		}
		
	}
		
	return 0;
	
}
