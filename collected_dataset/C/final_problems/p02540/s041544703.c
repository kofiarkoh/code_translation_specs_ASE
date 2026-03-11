#include<stdio.h>

int main(void){
  int n;
  scanf("%d",&n);
  int x[n],y[n];
  for(int i=0;i<n;i++){
    scanf("%d %d",&x[i],&y[i]);
  }
  for(int i=0;i<n;i++){
    int c=1;
     for(int j=0;j<n;j++){
    
       if(x[i]>x[j] && y[i]>y[j]){
         c++;
         int d=0;
         for(int q=0;q<n;q++){
           
           if(x[j]<x[q] && y[j]<y[q]){
         d++;
           }
         }
         if(d>1){
           c+=d-1;
         }
       
         }else if(x[i]<x[j] && y[i]<y[j]){
         c++;
         int d=0;
         for(int q=0;q<n;q++){
           if(x[j]>x[q] && y[j]>y[q]){
         d++;
           }
         }
         if(d>1){
           c+=d-1;
         }
       }
       
     }
    
    printf("%d\n",c);   
    
  }
  return 0;
}
  
