#include <stdio.h>

#define STRING_DATA 20000
#define LAKE 500

int stack_index[STRING_DATA];
int stack_area[STRING_DATA];
int stack_lake[LAKE];
int l = 0, pop_flag;

void push(int i) {
  stack_index[l] = i;
  l++;
}

int pop() {
  if(l != 0) {
    pop_flag = 0;
    l--;
    return stack_index[l];
  }
  else {
    pop_flag = 1;
    return 0;
  }
}

int main(void) {
  int k = 0, t, i, j = 0, max = 0;
  long int max_lake = 0;
  char in;

  for(i = 0 ; i < STRING_DATA ; i++) {
    stack_area[i] = 0;
  }
  
  while((in = getchar()) != EOF) {
    if(in == '\\') {
      push(k);
      max = k;
    }
    else if(in == '/') {
      t = pop();
      if(pop_flag != 1) {
	stack_area[t] = k - t;
	for(i = t + 1 ; i <= max ; i++) {
	  stack_area[t] += stack_area[i];
	  stack_area[i] = 0;
	}
      }
    }
    k++;
  }

  for(i = 0 ; i < k ; i++) {
    if(stack_area[i] != 0) {
      max_lake += stack_area[i];
      stack_lake[j] = stack_area[i];
      j++;
    }
  }
  
  printf("%ld\n", max_lake);
  printf("%d", j);
  for(i = 0 ; i < j ; i++) {
    printf(" ");
    printf("%d", stack_lake[i]);
  }
  printf("\n");
  return 0;
}

