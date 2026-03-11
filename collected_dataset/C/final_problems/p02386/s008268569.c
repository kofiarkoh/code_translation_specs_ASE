#include <stdio.h>

typedef struct {
    int top;
    int bottom;
    int right;
    int left;
    int front;
    int back;
} Dice;

void roll(Dice *d, char direction)
{
    int present_top = d->top;
    int present_bottom = d->bottom;
    int present_right = d->right;
    int present_left = d->left;
    int present_front = d->front;
    int present_back = d->back;

    if (direction == 'N') {
        d->top = present_front;
        d->bottom = present_back;
        d->front = present_bottom;
        d->back = present_top;
    } else if (direction == 'S') {
        d->top = present_back;
        d->bottom = present_front;
        d->front = present_top;
        d->back = present_bottom;
    } else if (direction == 'E') {
        d->top = present_left;
        d->bottom = present_right;
        d->right = present_top;
        d->left = present_bottom;
    } else {
        d->top = present_right;
        d->bottom = present_left;
        d->right = present_bottom;
        d->left = present_top;
    }
}

void revolve(Dice *d, char direction)
{
    int present_top = d->top;
    int present_bottom = d->bottom;
    int present_right = d->right;
    int present_left = d->left;
    int present_front = d->front;
    int present_back = d->back;

    if (direction == 'R') {
        d->front = present_right;
        d->back = present_left;
        d->right = present_back;
        d->left = present_front;
    } else {
        d->front = present_left;
        d->back = present_right;
        d->right = present_front;
        d->left = present_back;
    }
}

int is_same(Dice *d1, Dice *d2)
{
    int i;

    for (i = 0; i < 6; i++) {
        if (d1->top == d2->top) {
            int j;
            for (j = 0; j < 4; j++) {
                if (d1->front == d2->front) {
                    if (d1->bottom == d2->bottom && d1->right == d2->right
                        && d1->left == d2->left) return 1;
                }
                else revolve(d2, 'R');
            }
        }
        if (i % 2 == 0) roll(d2, 'N');
        else roll(d2, 'W');
    }

    return 0;
}

int main(void)
{
    int n;
    scanf("%d", &n);

    Dice dices[n];
    
    int flag = 0;
    int i;

    for (i = 0; i < n; i++) {
        int j;
        for (j = 0; j < 6; j++) {
            switch (j) {
                case 0 : scanf("%d", &dices[i].top); break;
                case 1 : scanf("%d", &dices[i].front); break;
                case 2 : scanf("%d", &dices[i].right); break;
                case 3 : scanf("%d", &dices[i].left); break;
                case 4 : scanf("%d", &dices[i].back); break;
                case 5 : scanf("%d", &dices[i].bottom); break;
            }
        }
        for (j = 0; j < i; j++) {
            if(flag = is_same(&dices[i], &dices[j]));
            break;
        }
        if (flag) break;
    }

    if (flag) puts("No");
    else puts("Yes");

    return 0;
}