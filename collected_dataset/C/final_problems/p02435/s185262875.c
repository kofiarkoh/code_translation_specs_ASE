#include <stdio.h>
#include <stdlib.h>

struct node
{
    int ele;
    struct node *next;
};

struct stack
{
    struct node *top;
};

typedef struct node * Node;
typedef struct stack * Stack;

void initialize(Stack *);
void push(Stack, int);
void top(Stack);
void pop(Stack);

int main(void)
{
    int n, q, i;
    int cmd, idx, val;

    scanf("%d %d", &n, &q);

    Stack *list = (Stack *) malloc(sizeof(Stack *) * n);
    for (i = 0; i < n; i++)
        initialize(list + i);
    
    for (i = 0; i < q; i++) {
        scanf("%d %d", &cmd, &idx);
        if (cmd == 0) {
            scanf("%d", &val);
            push(list[idx], val);
        } else if (cmd == 1) {
            top(list[idx]);
        } else if (cmd == 2) {
            pop(list[idx]);
        }
    }

    return 0;
}

void initialize(Stack *s)
{
    (*s) = (Stack) malloc(sizeof(struct stack));
    (*s)->top = NULL;
}

void push(Stack s, int x)
{
    Node tmp = (Node) malloc(sizeof(struct node));
    tmp->ele = x;
    tmp->next = s->top;
    s->top = tmp;
}

void top(Stack s)
{
    if (s->top)
        printf("%d\n", s->top->ele);
}

void pop(Stack s)
{
    if (s->top) {
        Node tmp = s->top;
        s->top = s->top->next;
        free(tmp);
    }
}
