#include <stdio.h>
int main(){

  int hen[12],i,count=0,next;

  for(i = 0; i < 12; i++){

    scanf("%d",&hen[i]);

  } 

  for(i = 1; i < 12; i++){

    if(hen[0] == hen[i]) count++;

  }


  if(count >= 3){

    count=0;

    if(hen[0] == hen[1]){

      if(hen[0] == hen[2]){

	if(hen[0] == hen[3]){

	  for(i = 5; i < 12; i++){

	    if(hen[4] == hen[i]) count++;
	    next=4;
	    
	  }

	} else {
	  
	  for(i = 4; i < 12; i++){
	    
	    if(hen[3] == hen[i]) count++;
	    next=3;
	    
	  }

	}

      } else {
	
	for(i = 3; i < 12; i++){
	  
	  if(hen[2] == hen[i]) count++;
	  next=2;
	  
	}
	
      }

    } else {

      for(i = 2; i < 12; i++){

	if(hen[1] == hen[i]) count++;
	next=1;
	
      }

    }

    if(count >= 3){

      count=0;

      if(hen[next] == hen[next+1]){

	if(hen[next] == hen[next+2]){

	  if(hen[next] == hen[next+3]){

	    for(i = next+5; i < 12; i++){

	      if(hen[next+4] == hen[i]) count++;

	    }

	  } else {

	    for(i = next+4; i < 12; i++){

	      if(hen[next+3] == hen[i]) count++;

	    }

	  }
	  
	} else {
	 
	  for(i = next+3; i < 12; i++){

	    if(hen[next+2] == hen[i]) count++;

	  }

	}

      } else {

	for(i = next+2; i < 12; i++){

	  if(hen[next+1] == hen[i]) count++;
	  next=1;

	}

      }

    }

    if(count >= 3){
      
      printf("yes\n");
      
      return 0;

    }

  }
    
  printf("no\n");

  return 0;
  
}