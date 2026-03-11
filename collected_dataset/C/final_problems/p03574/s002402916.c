#include <stdio.h>

int main(void){
  int s, w, sum, i, j, d, e;
  scanf("%d %d", &s, &w);
  char c[s+1][w+1];

  for(i = 0; i < s; i++){
    scanf("%s", c[i]);
  }

  for(i = 0; i < s; i++){
    for(j=0; j < w; j++){
      sum = 0;
      if(c[i][j] == '#') putchar('#');
      else{
	for(d = -1; d <= 1; d++){
	  for(e = -1; e <= 1; e++){
	    if(c[i+e][j+d] == '#'){
	      sum++;
	    }
	  }
	}	
	printf("%d", sum);
      }
    }
    putchar('\n');
  }
  return 0;
}

