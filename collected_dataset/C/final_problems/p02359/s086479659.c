#include<stdio.h>
#include<math.h>
#include<limits.h>
#define MAX 100000


int max(int a,int b){
 if(a < b)return b;
 return a;
}


int main(){
 int n,T,i,j,maxi;
 int x,y;
 int A[MAX];
 int B[MAX];
 int b,L,R;

 scanf("%d %d",&n,&T);
 b=floor(sqrt(n));

 for(i=0;i<=n;i++){
   A[i]=0;
   B[i]=-1;
 }


 for(i=0;i<n;i++){

   scanf("%d %d",&x,&y);

     /*update*/
     for(j=x;j<y;j++){
      A[j]++;
      B[j/b]=max(B[j/b],A[j]);
     }


    /*find*/
     maxi=-1;
     for(j=0;B[j]!=-1;j++)maxi=max(maxi,B[j]); 

 }

 printf("%d\n",maxi);

return 0;
}
