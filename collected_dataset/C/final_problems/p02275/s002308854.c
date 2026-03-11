#include<stdio.h>
#include<stdlib.h>



int main(){
int n,i,j;

int A[200000];
int C[10001]={0};



scanf("%d",&n);


for(i=0;i<n;i++){
 scanf("%d",&A[i]);
 C[A[i]]++;
 }


for(i=0;i<10001;i++){

if(C[i]!=0){for(j=0;j<C[i];j++){printf("%d ",i);}}

}

return 0;
}