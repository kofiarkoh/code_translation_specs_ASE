#include <stdio.h>
#include <stdlib.h>

int value[100];
int sp = 0;

void push(const int val)
{
    value[sp++] = val;
}

int pop()
{
    return value[--sp];
}

void operand(const char operand)
{
    int ch, val = operand - '0';
    while( (ch = getchar()) != ' ' ){
        val = val * 10 + (ch - '0'); 
    }
    push(val);
}

void operator(const char operator)
{
    int tmp1 = pop(), tmp2 = pop();

    switch( operator ){
    case '+': push(tmp2 + tmp1); break;
    case '-': push(tmp2 - tmp1); break;
    case '*': push(tmp2 * tmp1); break;
    default : break;
    }
}

int main()
{
    while( 1 ){
        int ch = getchar();
        if( ch == ' ' || ch == '\n' ) continue;
        if( ch == EOF ) break;
        
        switch( ch ){
        case '+': case '-': case '*': operator(ch); break;
        default : operand(ch); break;
        }
    }

    printf("%d\n", pop());

    return 0;
}