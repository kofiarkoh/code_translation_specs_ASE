#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdbool.h>

int max(int a,int b){if(a>b){return a;}return b;}
int min(int a,int b){if(a<b){return a;}return b;}
int sortfncsj(const void *a,const void *b){if(*(int *)a>*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}

int dx4[4]={1,-1,0,0};
int dy4[4]={0,0,1,-1};

int cti(char c){
  if(c=='i'){return 1;}
  if(c=='x'){return 10;}
  if(c=='c'){return 100;}
  if(c=='m'){return 1000;}
  return 0;
}

int sti(char s[]){
  int i=0,l=strlen(s),r=0,k=1;
  while(i<l){
    if(cti(s[i])!=0){
      //printf("%c %d\n",s[i],k);
      r+=cti(s[i])*k;
      k=1;
    }
    else{k=s[i]-'0';}
    i++;
  }
  return r;
}

char de[8]="mcxi";
void its(int a){
  //printf("<%d>",a);
  int k=1000,i=0,v;
  for(i=0;i<4;i++){
    v=a/k;
    if(v>=2){printf("%d",v);}
    if(v!=0){printf("%c",de[i]);}
    a%=k;k/=10;
  }
  printf("\n");
}

int main(){
  int i,j,n;
  char a[16],b[16];
  scanf("%d",&n);
  for(i=0;i<n;i++){
    scanf("%s%s",a,b);
    its(sti(a)+sti(b));
  }
  return 0;
}

