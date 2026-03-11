#include<stdio.h>
#define MAX 2147483647
#define NMAX 100001


int getsum(int A[],int x,int y){
int i;
int sum=0;

for(i=x;i<=y;i++){
 sum+=A[i];
}

return sum;
}



int main(){
int n,q,i;
int com,x,y;
int A[NMAX]={0};
int C[NMAX];
scanf("%d %d",&n,&q);

for(i=0;i<q;i++)C[i]=-1;

for(i=0;i<q;i++){
 scanf("%d %d %d",&com,&x,&y);
 switch(com){
  case 0:A[x]+=y;
         break;
  case 1:C[i]=getsum(A,x,y);
         break;
 }
}


for(i=0;i<q;i++){if(C[i]!=-1){printf("%d\n",C[i]);}}

return 0;
}
