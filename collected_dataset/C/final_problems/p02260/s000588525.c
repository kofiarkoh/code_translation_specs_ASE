#include<stdio.h>

void selectionSort(int a[],int n){
  int i,j,minj,temp,count=0;
  for(i=0; i<n; i++ ){
    minj=i ;
    for(j=i;j<n;j++){
      if(a[j]<a[minj]){
        minj =j;
      }
    }
  if(i != minj){    
        temp=a[i];
        a[i]=a[minj];
        a[minj]=temp;
        count++;

}


  }

    for(i=0;i<n-1;i++){
      printf("%d ",a[i]);
}
printf("%d\n",a[n-1]);
printf("%d\n",count);
}

int main(void){
  int n,i,count ;
  scanf("%d",&n);
  int a[n];
  for(i=0;i<n;i++){
    scanf("%d",&a[i]);
  }
  selectionSort(a,n);

return 0;
}

