#include<stdio.h>
int main()
{
   long num;//合計
    scanf("%ld", &num);
   
  int n100 = 0;
  int n1 = 0;
  
  
  //ここ違う
 // if(num < 1000){
  n1 = num % 100;
  n100 = num / 100;
  int n5 = n1 /5;
  if(n5 <= n100){
	printf("1");
    return 0;
  }else{
    printf("0");
    return 0;
  }
  int i=0;
  //n100個使って、n1にしたい
  int n2 = n1;
  int count = 0;
  
  for(i=0;i<n1;i++){
	if(n2 > 5){
     // printf("n2:%d\n",n2);
      n2= n2-5;
      count++;
    }else if(n2 > 4){
     // printf("n2:%d\n",n2);
        n2= n2-4;
      count++;
    }else if(n2 > 3)  {
    ///printf("n2:%d\n",n2);
      n2= n2-3;
      count++;
    }else if(n2 > 2){
    //  printf("n2:%d\n",n2);
      n2= n2-2;
      count++;
	}else if(n2 > 1){
     // printf("n2:%d\n",n2);
       n2= n2-1;
      count++;
    } 
  }

  if(count <= n100){
	printf("1");
  }else{
    printf("0");
  }
    return 0;
}