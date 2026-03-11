#include<stdio.h>
#include<stdlib.h>
#include<string.h>


typedef struct card{
	char key[25];
	int data;
	struct card *left;
	struct card *right;
}card;

card * makecard(char str[],int x){
	card *new=malloc(sizeof(card));
	new->data=x;
	new->left=NULL;
	new->right=NULL;
	strcpy(new->key,str);
	return new;
}

void insert(card *S[],char str[],int x){
	card *new=makecard(str,x);
	card **p=&S[str[0]-'a'];
	
	while((*p)!=NULL){
		if(strcmp(str,(*p)->key)==0){
			(*p)->data=x;
			return;
		}
		
		else if(strcmp(str,(*p)->key)>0)p=&((*p)->right);
		else if(strcmp(str,(*p)->key)<0)p=&((*p)->left);
	}
		
	(*p)=new;
	
	return;	
}


int get(card *S[],char str[]){
	card *p=S[str[0]-'a'];
	
	while(p!=NULL){
		if(strcmp(str,p->key)==0)return p->data;
		else if(strcmp(str,p->key)>0)p=p->right;
		else if(strcmp(str,p->key)<0)p=p->left;
	}
	
}



int main(){
	int q,com,i,x;
	char temp[25];
	card *S[26];
	card *p;
	
	for(i=0;i<26;i++)S[i]=NULL;
	
	scanf("%d",&q);
	for(i=0;i<q;i++){
		scanf("%d",&com);
		
		if(com==0){
			scanf("%s %d",temp,&x);
			insert(S,temp,x);
		}
		
		if(com==1){
			scanf("%s",temp);
			printf("%d\n",get(S,temp));
		}
		
	}
	
	return 0;
	
}
