#include <stdio.h>
#include <string.h>

int main(int argc,char* argv[]){
  int array[12];
  int post_array[12];
  int i,j;

  for(;;){
    int n;
    int count = 0;
    scanf("%d",&n);
    if(n == 0){break;}

    for(i = 0; i < n; i++){
      scanf("%d",&array[i]);
    }
    
    for(;;){
      int ok = 1;
      memset(post_array,0,sizeof(post_array));

      for(i = 0; i < n; i++){
	for(j = 0; j < n; j++){
	  if(array[i] == array[j]){
	    post_array[i]++;
	  }
	}
      }

      for(i = 0; i < n; i++){
	if(post_array[i] != array[i]){
	  ok = 0;
	  break;
	}
      }

      if(ok == 0){
	for(i = 0; i < n; i++){
	  array[i] = post_array[i];
	}
	count++;
	continue;
      }else{
	break;
      }
    }

    printf("%d\n",count);
    for(i = 0; i < n; i++){
      if(i != 0){printf(" ");}
      printf("%d",array[i]);
    }
    printf("\n");
  }
  
  return 0;
  
}