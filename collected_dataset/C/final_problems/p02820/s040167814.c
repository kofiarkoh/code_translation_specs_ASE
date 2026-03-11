#include<stdio.h>
int main(void)
{
  int n,k,r,s,p,i,f,u[100000];
  char t[100000];
  scanf("%d %d",&n,&k);
  scanf("%d %d %d",&r,&s,&p);
  scanf("%s",t);
  f=0;
  /*printf("%d %d\n",r,p);*/
  /*printf("%c %c %c\n",t[0],t[1],t[2]);*/
  for(i=0;i<k;i++){
    if(t[i]=='r'){
      f=f+p;
      u[i]=1;
      /*printf("%d p\n",f);*/
    }
    else if(t[i]=='s'){
      f=f+r;
      u[i]=2;
      /*printf("%d r\n",f);*/
    }else{
      f=f+s;
      u[i]=3;
      /*printf("%d s\n",f);*/
    }
    /*printf("%d\n",f);*/
  }
  /*printf("%d\n",f);*/
  for(i=k;i<n;i++){
    /*printf("%d\n",f);*/
    /* if(t[i]!=t[i-k]||(t[i]==t[i-k]&&s[i-k]!=11&&s[i-k]!=12&&s[i-k]!=13)){*/
    if(t[i]=='r'&&u[i-k]!=1){
      f=f+p;
      u[i]=1;
    }
    else if(t[i]=='r'&&u[i-k]==1){
      u[i]=0;
    }
    else if(t[i]=='s'&&u[i-k]!=2){
      f=f+r;
      u[i]=2;
    }
    else if(t[i]=='s'&&u[i-k]==2){
      u[i]=0;
    }
    else if(t[i]=='p'&&u[i-k]!=3){
      f=f+s;
      u[i]=3;
    }
    else{
      u[i]=0;
    }
    /* printf("%d\n",f);*/
  }
  printf("%d\n",f);
  return 0;
}
