#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <limits.h>
#define swap(type,a,b) do{type t=a;a=b;b=t;}while(0);
#define MAX(a,b) (((a)>(b))?(a):(b))
#define MIN(a,b) (((a)<(b))?(a):(b))
#define ll long long
#define INF 100000000
#define FOR(i,a,n) for(i=(a);i<(n);i++)
int comp(const void* a,const void* b){
	return *(int*)a-*(int*)b;
}
void fill(int a[],int b,int c){
	int i;
	FOR(i,0,b) a[i]=c;
	return;
}
typedef struct node{
	struct node *right;
	struct node *left;
	int data;
} Node;
Node* add(Node *n,int a){
	if(n==NULL){
		Node *m=malloc(sizeof(Node));
		m->data=a;
		m->right=m->left=NULL;
		return m;
	}
	else{
		if(n->data<a) n->right=add(n->right,a);
		else n->left=add(n->left,a);
		return n;
		}
}
int find(Node *n,int a){
	if(n==NULL) return 0;
	if(n->data==a) return 1;
	if(n->data<a) return find(n->right,a);
	else return find(n->left,a);
}
int a[100000];
int main(void)
{
	int n,res=0,i;
	Node *k=NULL;
	scanf("%d",&n);
	FOR(i,0,n){
		scanf("%d",&a[i]);
		if(i%2) k=add(k,a[i]);
	}
	qsort(a,n,sizeof(int),comp);
	for(i=0;i<n;i+=2){
		if(find(k,a[i])) res++;
	}
	printf("%d\n",res);
	return 0;
}
    
    
