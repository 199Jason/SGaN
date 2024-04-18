% SPACECRAFT GUIDANCE AND NAVIGATION (2022/2023)
% Assignment #1 ex 1.1, Lagrange Points
% Author: Spinelli Jason

%% Lagrange point
clearvars; close all; clc; 
Data.mu = 3.0359*1e-6;
LP = lagrangePoints(Data);
L1 = LP{1}; L2 = LP{2}; L3 = LP{3}; 
L4 = LP{4}; L5 = LP{5}; 

figure % plot the libration points
plot(L1(1), 0, 'ko'); 
text(L1(1)-0.1,0.1,'L_1','VerticalAlignment','top','HorizontalAlignment','right');
hold on; grid on;
plot(L2(1), 0, 'ko');
text(L2(1)+0.1,0.1,'L_2','VerticalAlignment','top','HorizontalAlignment','left');
plot(L3(1), 0, 'ko');
text(L3(1),0,'L_3','VerticalAlignment','top','HorizontalAlignment','center');
plot(L4(1), L4(2), 'ko');
text(L4(1),L4(2),'L_4','VerticalAlignment','top','HorizontalAlignment','center');
plot(L5(1), L5(2), 'ko');
text(L5(1),L5(2),'L_5','VerticalAlignment','bottom','HorizontalAlignment','center');

%plot the planets
mu = Data.mu;
plot(-mu,  0,'y*','LineWidth',2);
text(-mu,  0,'Sun','VerticalAlignment','top','HorizontalAlignment','center');
plot(1-mu, 0,'b.','LineWidth',1);
text(1-mu, 0,'Earth','VerticalAlignment','top','HorizontalAlignment','center');
xlabel('x [nondimensional]','Interpreter','latex')
ylabel('y [nondimensional]','Interpreter','latex')
title('Lagrange Points','Interpreter','latex')
axis equal
axis(1.1*axis)

figure % zoom of the L2 lagrange point
plot(L1(1), 0, 'ko'); 
text(L1(1)-0.001,0.1,'L_1','VerticalAlignment','top','HorizontalAlignment','right');
hold on; grid on;
plot(L2(1), 0, 'ko');
text(L2(1)+0.001,0.1,'L_2','VerticalAlignment','top','HorizontalAlignment','left');
plot(1-mu, 0,'b*','LineWidth',3);
text(1-mu, 0,'Earth','VerticalAlignment','top','HorizontalAlignment','center');
xlabel('x [nondimensional]','Interpreter','latex')
ylabel('y [nondimensional]','Interpreter','latex')
title('L1 and L2 points','Interpreter','latex')

%% function
function LP = lagrangePoints(Data)
% lagrangePoints - find the position of the lagrange points
%
% PROTOTYPE
%    out = lagrangePoints(Data)
%
% INPUTS:
%   Data       [---]       system Data               [struct]
%
%  (For this code to work, Data struct has to include the value of:
%           mu  - system mass parameter [(m(^3))/(s^(−2))]
%   )
% OUTPUTS:
%   out  [---]       contains the position of all Lagrange points 
%
% CALLED FUNCTIONS: [-]
%
% CONTRIBUTOR:
%   Spinelli Jason              10618465
%
% VERSIONS
%   2022-11-08: Release
%
% -------------------------------------------------------------------------

% initialize data
mu = Data.mu;

% Considering a situation where the celestial bodies are on the same
% plane (z = 0), Lagrange points 1, 2 and 3 are located on the x axis so 
% the only unknown component is the x value. Lagrange points 3 and 4 are 
% located on the same x-component, but the value of the y-compoment is
% opposite betwen each other
% find L1
 y1=@(x) x-(1-mu)/(x+mu)^2+mu/(x-1+mu)^2;
 L1=fzero(y1,0);
 
% find L2
 y2=@(x) x-(1-mu)/(x+mu)^2-mu/(x-1+mu)^2;
 L2=fzero(y2,0);
 
% find L3
 y3=@(x) x+(1-mu)/(x+mu)^2+mu/(x-1+mu)^2;
 L3=fzero(y3,0);
 
% find component of L4
 L4x=0.5-mu;
 L4y=0.5*sqrt(3);

% find component of L5
 L5x=0.5-mu;
 L5y=-0.5*sqrt(3);

 % define output:   x;   y;  z
 LP(1,1)   =    {[ L1;   0;  0]};
 LP(2,1)   =    {[ L2;   0;  0]}; 
 LP(3,1)   =    {[ L3;   0;  0]};
 LP(4,1)   =    {[L4x; L4y;  0]};
 LP(5,1)   =    {[L5x; L5y;  0]};

end