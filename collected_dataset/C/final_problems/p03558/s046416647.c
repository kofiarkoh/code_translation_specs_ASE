#include<stdio.h>
int main(){
  int k,n[100001]={0},q[100001]={0},i=0,j=1,l,x;
  scanf("%d",&k);
  n[1]=q[0]=1;
  while(!n[0]){
    for(l=i;l<j;l++){x=q[l]*10%k;while(!n[x]){n[x]=n[q[i]];q[j]=x;j++;x=x*10%k;}}
    for(l=i;n[q[i]]==n[q[l]];l++)if(!n[(q[l]+1)%k]){n[(q[l]+1)%k]=n[q[i]]+1;q[j]=(q[l]+1)%k;j++;}
    i=l;
  }
  printf("%d",n[0]);
}
