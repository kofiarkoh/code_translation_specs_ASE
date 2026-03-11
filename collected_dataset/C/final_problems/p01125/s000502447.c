#include<stdio.h>

int main()
{
  
  while(1){
    int now[21][21] = {0};
    int x, y, n, jewelry, i, j, cnt = 0, nowx = 10, nowy = 10;
    char derection, dummy;

    scanf("%d", &jewelry);
    if(jewelry == 0) break;

    for(i = 0; i < jewelry; i++){
      scanf("%d%d", &x, &y);
      now[x][y] = 1;
  
    }

    scanf("%d", &n);
    for(i = 0; i < n; i++){
      scanf("%c", &dummy);
      scanf("%c %d", &derection, &x);

      for(j = 0; j < x; j++){
    	switch(derection){
      
    	case 'N':
    	  nowy++;
    	  break;

    	case 'S':
    	  nowy--;
    	  break;

    	case 'W':
    	  nowx--;
    	  break;

    	case 'E':
    	  nowx++;
    	  break;
      
    	}

	if(now[nowx][nowy]){
	  cnt++;
	  now[nowx][nowy] = 0;
	}	
      }
    }
    
    if(cnt == jewelry) printf("Yes\n");
    else printf("No\n");

  }
  
  return 0;
  
  
}