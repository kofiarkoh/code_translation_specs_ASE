#include<stdio.h>

int main(void)
{
  
  int number;
  int array[10*10*10*10*10];
  int i;
  int j;
  int hp = 0;
  int totalhp = 0;
  int temp;
  int k = 0;

  scanf("%d",&number);

  for(i=0;i<number;i++){
    scanf("%d",&array[i]);
  }

  i = 0;

  while(i < number-1){
    if(array[i] > array[i+1]){
      for(j=i;j<number;j++){
	if(array[j+1]-array[j] == 1 || j == number-1){
	  if(hp > array[j])
	    totalhp += array[j];
	  else 
	    totalhp += hp;

	  if(j == number-1)
	    number-=2;
	  i = k;
	  hp = 0;
	  break;
	}
	else{
	  hp += array[j+1];
	  temp = array[j];
	  array[j] = array[j+1];
	  array[j+1] = temp;
	}
      }	
    }

    else if(array[i] == i+1){
      k++;
      i++;
    }

    else
      i++;
  }

  printf("%d\n",totalhp);
  
  return 0; 
  
}