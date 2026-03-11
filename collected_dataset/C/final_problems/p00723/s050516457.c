#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef struct node{
  struct node *next['z'-'a'+1];
}node;

node *_N;
int _count;

node *allocNode(){
  return &_N[_count++];
}

#define MAX (72*(72*6)+2)
void init(){
  _N=(node *)calloc(sizeof(node),MAX);
  _count=0;
}
void reinit(){
  memset(_N,0,sizeof(node)*_count);
  _count=0;
}

node *get(node *s,const char *p){
  while(*p!='\0'){
    if(s==NULL) return NULL;
    s=s->next[*(p++)-'a'];
  }
  return s;
}

int set(node *s,const char *p){
  int ret=0;
  while(*p!='\0'){
    if(s->next[*p-'a']==NULL){
      s->next[*p-'a']=allocNode();
      ret=1;
    }
    s=s->next[*(p++)-'a'];
  }
  return ret;  
}

void makestr(char *dist,char *s1,char *s2){
  strcpy(dist,s1);
  strcat(dist,s2);
}

int main(){
  int n;
  init();
  scanf("%d ",&n);
  while(n-->0){
    char buff[100];
    char buff2[100];
    node *s=allocNode();
    int len,i,ans=0;
    char f[100],*b,*fr,br[100];

    len=strlen(gets(buff));
    for(i=0;i<len;i++){
      buff2[i]=buff[len-1-i];
      br[i]=buff[len-1-i];
    }
    buff2[len]='\0';
    b=buff;
    fr=&buff2[len];

    ans+=set(s,buff);
    ans+=set(s,buff2);

    for(i=1;i<len;i++){
      char str[100];
      f[i-1]=buff[i-1];f[i]='\0';
      br[len-1-(i-1)]='\0';
      fr--;b++;
      //printf("%s(%s) + %s(%s)\n",f,fr,b,br);
      makestr(str,fr,b ); ans+=set(s,str);
      makestr(str,fr,br); ans+=set(s,str);
      makestr(str,f ,br); ans+=set(s,str);
      makestr(str,b ,f ); ans+=set(s,str);
      makestr(str,br,f ); ans+=set(s,str);
      makestr(str,b ,fr); ans+=set(s,str);
    }
    
    printf("%d\n",ans);

    if(n!=0) reinit();
  }
  return 0;
}