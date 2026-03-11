#include<stdio.h>
#define MAX 100000

int max(int a,int b){
 if(a>b)return a;

return b;
}



void func(int N,int W,int v[],int w[]){
int X[MAX]={0};
int Y[MAX]={0};
int i,j;

for(i=1;i<=W;i++){X[i]=-1;Y[i]=-1;}

X[w[0]]=v[0];


for(i=1;i<N;i++){

 if(i%2==1){

   for(j=0;j<=W;j++){
    if(X[j]!=-1){
       Y[j]=max(X[j],Y[j]);
       if(j+w[i]<=W)Y[j+w[i]]=X[j]+v[i];
       
    }
   }

 }

 else if(i%2==0){

   for(j=0;j<=W;j++){
    if(Y[j]!=-1){
       X[j]=max(X[j],Y[j]);
       if(j+w[i]<=W)X[j+w[i]]=Y[j]+v[i];
       
    }
   }

 }




 

}/*たて*/

if(N%2==0)for(i=0;i<W;i++)j=max(Y[i],Y[i+1]);

if(N%2==1)for(i=0;i<W;i++)j=max(X[i],X[i+1]);

printf("%d\n",j);

return;
}

int main(){
int N,W;
int v[MAX];
int w[MAX];
int i;

scanf("%d %d",&N,&W);

for(i=0;i<N;i++)scanf("%d %d",&v[i],&w[i]);

func(N,W,v,w);

return 0;
}
