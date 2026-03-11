#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node{
	struct node*oya,*ko[2];
	char key[30];
	int val,p,idx;
}node;

void rot(node*v,int d){
	node*nnn=v->ko[d];
	
	v->ko[d]=nnn->ko[d^1];
	if(v->ko[d])v->ko[d]->oya=v;
	
	v->oya->ko[v->oya->ko[1]==v]=nnn;
	nnn->oya=v->oya;
	
	nnn->ko[d^1]=v;
	v->oya=nnn;  
}
void ins(node*v,node*oya,int dir,node*n){
	if(!v){
		n->oya=oya;
		oya->ko[dir]=n;
	}else{
		int d=strcmp(v->key,n->key)<0||(strcmp(v->key,n->key)==0&&v->idx<n->idx);
		ins(v->ko[d],v,d,n);
		if(v->ko[d]->p > v->p)rot(v,d);
	}
}
void del(node*v){
	while(v){
		if(v->ko[0]){
			if(v->ko[1])rot(v,v->ko[1]->p > v->ko[0]->p);
			else rot(v,0);
		}else{
			if(v->ko[1])rot(v,1);
			else v=v->oya->ko[v->oya->ko[1]==v]=0;
		}
	}
}
void f(node*v,char*s,char*t,int mode){
	if(!v)return;
	if(strcmp(s,v->key)<=0)f(v->ko[0],s,t,mode);
	if(strcmp(s,v->key)<=0&&strcmp(v->key,t)<=0){
		if(mode==0)printf("%d\n",v->val);
		if(mode==2)printf("%s %d\n",v->key,v->val);
	}
	if(strcmp(v->key,t)<=0)f(v->ko[1],s,t,mode);
	if(mode==1&&strcmp(s,v->key)<=0&&strcmp(v->key,t)<=0)del(v);
}

node root;
int main(){
	int q;
	scanf("%d",&q);
	for(int i=0;i<q;i++){
		int t;
		char s[30];
		scanf("%d%s",&t,s);
		if(t==0){
			int x;
			scanf("%d",&x);
			node*v=calloc(1,sizeof(node));
			memcpy(v->key,s,20);
			v->val=x;
			v->idx=i;
			v->p=rand();
			ins(root.ko[0],&root,0,v);
		}else if(t==1){
			f(root.ko[0],s,s,0);
		}else if(t==2){
			f(root.ko[0],s,s,1);
		}else{
			char t[30];
			scanf("%s",t);
			f(root.ko[0],s,t,2);
		}
	}
}
