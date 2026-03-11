#include <stdio.h>
#include <stdlib.h>
 
#define CLEAN 0
#define DIRTY 1
#define SIZE  1401
#define INITIAL_STACK_SIZE 10
 
int table[SIZE][SIZE] = {};
int board[SIZE][SIZE] = {};
 
typedef struct Rectangle {
  int height;
  int pos;
} Rectangle;
 
typedef struct _stack {
  int size;
  int max;
  Rectangle *stack;
} *Stack;
 
Stack stack_init() {
  Stack stack = (Stack)malloc(sizeof(struct _stack));
  stack->size = 0;
  stack->max = INITIAL_STACK_SIZE;
  stack->stack = (Rectangle *)malloc(sizeof(Rectangle)*INITIAL_STACK_SIZE);
  return stack;
}
 
Rectangle empty_rectangle() {
  return (Rectangle) {
    .height = 0,
    .pos = 0
  };
}
 
Rectangle top(Stack stack) {
  if (stack->size <= 0) {
    return empty_rectangle();
  } else
    return stack->stack[stack->size-1];
}
 
Rectangle pop(Stack stack) {
  if (stack->size <= 0) {
    return empty_rectangle();
  } else
    return stack->stack[--(stack->size)];
}
 
Rectangle push(Rectangle item, Stack stack) {
  int i;
  if (stack->size < stack->max) {
    stack->stack[(stack->size)++] = item;
  } else {
    Rectangle *newStack = (Rectangle *)malloc(sizeof(int) * stack->max * 2);
 
    if (newStack == NULL) {
      perror("malloc failed.\n");
      exit(-1);
    }
      
    for (i=0; i<stack->size; i++)
      newStack[i] = stack->stack[i];
 
    newStack[stack->size++] = item;
    stack->max *= 2;
    free(stack->stack);
    stack->stack = newStack;
  }
 
  return item;
}
 
int empty(Stack stack) {
  return !stack->size;
}
 
void free_stack(Stack stack) {
  free(stack);
}
 
void create_table(int h, int w) {
  int i, j;
 
  for (i=0; i<w; i++)
    table[0][i] = (board[0][i] ? 0 : 1);
   
  for (i = 1; i < h; i++) {
    for (j = 0; j < w; j++) {
      if (board[i][j] == CLEAN)
    table[i][j] = table[i-1][j] + 1;
      else
    table[i][j] = 0;
    }
  }
}
 
int rectangleAreaMaxRow(int row[],int w) {
  Stack s = stack_init();
  int i, max = 0;
  row[w] = DIRTY;
 
  for (i=0; i<=w; i++) {
    Rectangle r;
    r.height = row[i];
    r.pos = i;
     
    if (empty(s) || top(s).height < r.height) {
      push(r, s);
    } else {
      int target = i;
      while (!empty(s) && top(s).height >= r.height) {
        Rectangle pre = pop(s);
        int area = pre.height * (i-pre.pos);
        max = (area > max ? area : max);
        target = pre.pos;
      }
    r.pos = target;
    push(r, s);
    }
  }
 
  free_stack(s);
  return max;
}
   
int rectangleAreaMax(int h, int w)
{
  int i, max = 0, temp;
  for (i=0; i<h; i++) {
    temp = rectangleAreaMaxRow(table[i], w);
    max = (temp > max ? temp : max);
  }
 
  return max;
}
 
int main() {
  int i, j, h, w;
 
  scanf("%d %d", &h, &w);
  for (i=0; i<h; i++)
    for (j=0; j<w; j++)
      scanf("%d", &board[i][j]);
 
  create_table(h, w);
   
  printf("%d\n", rectangleAreaMax(h, w));
   
  return 0;
}