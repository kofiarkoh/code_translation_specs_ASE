#include <stdio.h>
#include <stdio.h>

void SimSelSort(int num[ ], int n,int S);

/*
int main(void){
  int N;
  int S;
  int a[100000] = {};
  int i=0; 

  while(i<100000){
    a[i] = rand()%9+1;
    ++i;
  }
  printf("N select number");
  scanf("%d",&N);

  printf("S select number");
  scanf("%d",&S);

  SimSelSort(a,N,S);
  
  

}
*/


int main(void){
  int a[100000]={};
  int N;
  int S;
  int b;
  int i=0;

  scanf("%d",&N);

  scanf("%d",&S);

  for(i;i<N;i++){

   scanf("%d",&b);
   a[i]=b;
  }
 


  SimSelSort(a,N,S);
  
  

}



void SimSelSort(int num[ ], int n,int S)
{
  int i, j, k, min, temp;
  int time=0;
  int sum=0;

    for (i = 0; i < n - 1; i++) {
        min = num[i];                
        k = i;                       
        for (j = i + 1; j < n; j++) {
            if (num[j] < min) {      
                min = num[j];        
                k = j;                
            }
        }                             
        temp = num[i];                
        num[i] = num[k];
        num[k] = temp;
    }
  
    for(n; 0<n; --n){
      
      time++;
      sum += num[n-1];

      //printf("%d ",sum);

      if(S<sum){
	printf("%d\n",time);
	break;
      }else if(n==0){

      printf("0\n");


      }    
       
    }  
       
     if(n==0){

      printf("0\n");


      }   

}