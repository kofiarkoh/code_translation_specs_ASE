#include<stdio.h>
 
int main(){
 
  int i,j,n,mini,si,a1[37],a2[37],count=1,f=0;
  char sc,c1[37],c2[37];
 
  //load N
  scanf("%d",&n);
 
  //load number
  for(i=1;i<=n;i++){
    scanf(" %c%d",&c1[i],&a1[i]);
    a2[i]=a1[i];
    c2[i]=c1[i];
  }
 
 
 
  //BubbleSort
  for(i=1;i<=n;i++){
    for(j=n;j>=i+1;j--){
      //swap
      if(a1[j]<a1[j-1]){
        si=a1[j];
        a1[j]=a1[j-1];
        a1[j-1]=si;
    sc=c1[j];
    c1[j]=c1[j-1];
    c1[j-1]=sc;
 
    }
    }
  }
  //print
  for(i=1;i<=n;i++){
    printf("%c%d",c1[i],a1[i]);
    if(i!=n){
      printf(" ");
    }
  }
    printf("\n");
 
    //stability
    printf("Stable\n");
 
  //SelectionSort
  for(i=1;i<=n;i++){
    mini=i;
    for(j=i;j<=n;j++){
      if(a2[j]<a2[mini]){
        mini=j;
      }
    }
      //swap
      si=a2[i];
      a2[i]=a2[mini];
      a2[mini]=si;
      sc=c2[i];
      c2[i]=c2[mini];
      c2[mini]=sc;
  }
 
 
  //print
  for(i=1;i<=n;i++){
    printf("%c%d",c2[i],a2[i]);
    if(i!=n){
      printf(" ");
    }
  }
  printf("\n");
 
  //stability
  f=0;
  for(i=1;i<=n;i++){
    if(c2[i]!=c1[i]){
      printf("Not stable\n");
      f=1;
      break;
    }
  }  
  if(f==0)printf("Stable\n");
   
   
   
 
  return 0;
 
}
